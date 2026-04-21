use std::collections::BTreeSet;

use xdr_parser::ast::{Definition, Size, StructMember, Type, Union, UnionCaseValue};
use xdr_parser::types::{is_builtin_type, TypeInfo};

use crate::naming::{field_name, type_name};
use crate::output::{LazyCaseOutput, LazyScanStepOutput, LazyTypeOutput, LazyValueOutput};

pub struct LazyCodegen<'a> {
    type_info: &'a TypeInfo,
}

#[derive(Clone, Debug)]
enum FixedLen {
    Const(u32),
    Expr(String),
}

impl FixedLen {
    fn render(&self) -> String {
        match self {
            Self::Const(value) => value.to_string(),
            Self::Expr(expr) => expr.clone(),
        }
    }

    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Const(lhs), Self::Const(rhs)) => lhs
                .checked_add(rhs)
                .map(Self::Const)
                .unwrap_or_else(|| Self::Expr(format!("checked_add_u32({lhs}, {rhs})?"))),
            (Self::Const(0), rhs) => rhs,
            (lhs, Self::Const(0)) => lhs,
            (lhs, rhs) => Self::Expr(format!(
                "checked_add_u32({}, {})?",
                lhs.render(),
                rhs.render()
            )),
        }
    }

    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Const(lhs), Self::Const(rhs)) => lhs
                .checked_mul(rhs)
                .map(Self::Const)
                .unwrap_or_else(|| Self::Expr(format!("checked_mul_u32({lhs}, {rhs})?"))),
            (Self::Const(0), _) | (_, Self::Const(0)) => Self::Const(0),
            (Self::Const(1), rhs) => rhs,
            (lhs, Self::Const(1)) => lhs,
            (lhs, rhs) => Self::Expr(format!(
                "checked_mul_u32({}, {})?",
                lhs.render(),
                rhs.render()
            )),
        }
    }
}

impl<'a> LazyCodegen<'a> {
    pub fn new(type_info: &'a TypeInfo) -> Self {
        Self { type_info }
    }

    pub fn scan_steps(&self, members: &[StructMember], index: usize) -> Vec<LazyScanStepOutput> {
        let mut steps = Vec::new();
        let mut fixed_prefix: Option<FixedLen> = None;

        for member in &members[..index] {
            if let Some(len_expr) = self.fixed_len_expr(&member.type_) {
                fixed_prefix = Some(match fixed_prefix.take() {
                    Some(prefix) => prefix.add(len_expr),
                    None => len_expr,
                });
                continue;
            }

            if let Some(prefix) = fixed_prefix.take() {
                steps.push(LazyScanStepOutput::fixed(prefix.render()));
            }
            steps.push(LazyScanStepOutput::variable(self.type_output(&member.type_)));
        }

        if let Some(prefix) = fixed_prefix {
            steps.push(LazyScanStepOutput::fixed(prefix.render()));
        }

        steps
    }

    pub fn type_output(&self, type_: &Type) -> LazyTypeOutput {
        match type_ {
            Type::Int => LazyTypeOutput::I32,
            Type::UnsignedInt => LazyTypeOutput::U32,
            Type::Hyper => LazyTypeOutput::I64,
            Type::UnsignedHyper => LazyTypeOutput::U64,
            Type::Bool => LazyTypeOutput::Bool,
            Type::Float => LazyTypeOutput::F32,
            Type::Double => LazyTypeOutput::F64,
            Type::OpaqueFixed(size) => LazyTypeOutput::FixedOpaque {
                size: self.size_expr(size),
            },
            Type::OpaqueVar(max) => LazyTypeOutput::VarOpaque {
                max_size: self.max_size_expr(max.as_ref()),
            },
            Type::String(max) => LazyTypeOutput::String {
                max_size: self.max_size_expr(max.as_ref()),
            },
            Type::Ident(name) => {
                let rust_name = type_name(name);
                if let Some(Definition::Typedef(typedef)) = self.type_info.definitions.get(&rust_name)
                {
                    if is_builtin_type(&typedef.type_) {
                        return self.type_output(&typedef.type_);
                    }
                }
                LazyTypeOutput::Named { name: rust_name }
            }
            Type::Optional(inner) => LazyTypeOutput::Optional {
                inner: Box::new(self.type_output(inner)),
            },
            Type::Array { element_type, size } => LazyTypeOutput::Array {
                element: Box::new(self.type_output(element_type)),
                size: self.size_expr(size),
            },
            Type::VarArray {
                element_type,
                max_size,
            } => LazyTypeOutput::VarArray {
                element: Box::new(self.type_output(element_type)),
                max_size: self.max_size_expr(max_size.as_ref()),
            },
        }
    }

    pub fn value_output(&self, type_: &Type) -> LazyValueOutput {
        match type_ {
            Type::Int => LazyValueOutput::I32,
            Type::UnsignedInt => LazyValueOutput::U32,
            Type::Hyper => LazyValueOutput::I64,
            Type::UnsignedHyper => LazyValueOutput::U64,
            Type::Bool => LazyValueOutput::Bool,
            Type::Ident(name) => {
                let rust_name = type_name(name);
                if let Some(Definition::Typedef(typedef)) = self.type_info.definitions.get(&rust_name)
                {
                    if is_builtin_type(&typedef.type_) {
                        return self.value_output(&typedef.type_);
                    }
                }
                LazyValueOutput::NamedEnum { name: rust_name }
            }
            _ => unreachable!("unsupported lazy value type"),
        }
    }

    pub fn union_arm_method_name(
        &self,
        value: &Union,
        arm_index: usize,
        case_index: usize,
    ) -> String {
        let case_name = self.case_name(value, arm_index, case_index);
        format!("as_{}", field_name(&case_name))
    }

    pub fn union_case_output(
        &self,
        value: &Union,
        arm_index: usize,
        case_index: usize,
    ) -> LazyCaseOutput {
        let discriminant_is_builtin = self.union_discriminant_is_builtin(value);
        let discriminant_prefix = self.union_discriminant_prefix(value);
        let case = &value.arms[arm_index].cases[case_index].value;
        match case {
            UnionCaseValue::Ident(name) => {
                let variant_name = type_name(&strip_prefix(name, &discriminant_prefix));
                if discriminant_is_builtin {
                    LazyCaseOutput::BareIdent { name: variant_name }
                } else {
                    LazyCaseOutput::QualifiedVariant {
                        type_name: self.value_output(&value.discriminant.type_).eager_type(),
                        variant_name,
                    }
                }
            }
            UnionCaseValue::Literal(value_) => {
                if discriminant_is_builtin {
                    LazyCaseOutput::Literal { value: *value_ }
                } else {
                    LazyCaseOutput::QualifiedLiteral {
                        type_name: self.value_output(&value.discriminant.type_).eager_type(),
                        value: *value_,
                    }
                }
            }
        }
    }

    fn case_name(&self, value: &Union, arm_index: usize, case_index: usize) -> String {
        let discriminant_prefix = self.union_discriminant_prefix(value);
        let case = &value.arms[arm_index].cases[case_index].value;
        match case {
            UnionCaseValue::Ident(name) => type_name(&strip_prefix(name, &discriminant_prefix)),
            UnionCaseValue::Literal(value_) => format!("V{value_}"),
        }
    }

    fn union_discriminant_is_builtin(&self, value: &Union) -> bool {
        is_builtin_type(&value.discriminant.type_)
            || matches!(&value.discriminant.type_, Type::Ident(name) if {
                self.type_info
                    .definitions
                    .get(&type_name(name))
                    .map(|definition| matches!(definition, Definition::Typedef(typedef) if is_builtin_type(&typedef.type_)))
                    .unwrap_or(false)
            })
    }

    fn union_discriminant_prefix(&self, value: &Union) -> String {
        if self.union_discriminant_is_builtin(value) {
            String::new()
        } else {
            self.type_info
                .discriminant_enum(&value.discriminant.type_)
                .map(|enum_| enum_.member_prefix.clone())
                .unwrap_or_default()
        }
    }

    fn size_expr(&self, size: &Size) -> String {
        self.type_info.size_to_literal(size)
    }

    fn max_size_expr(&self, size: Option<&Size>) -> String {
        size.map(|value| self.size_expr(value))
            .unwrap_or_else(|| "u32::MAX".to_string())
    }

    fn fixed_len_expr(&self, type_: &Type) -> Option<FixedLen> {
        self.fixed_len_expr_inner(type_, &mut BTreeSet::new())
    }

    fn fixed_len_expr_inner(&self, type_: &Type, visiting: &mut BTreeSet<String>) -> Option<FixedLen> {
        match type_ {
            Type::Int | Type::UnsignedInt | Type::Bool => Some(FixedLen::Const(4)),
            Type::Hyper | Type::UnsignedHyper => Some(FixedLen::Const(8)),
            Type::Float | Type::Double => None,
            Type::OpaqueFixed(size) => {
                let size_expr = self.size_expr(size);
                if let Ok(size) = size_expr.parse::<u32>() {
                    let padded = size.checked_add((4 - (size % 4)) % 4)?;
                    Some(FixedLen::Const(padded))
                } else {
                    Some(FixedLen::Expr(format!("fixed_opaque_len({size_expr})?")))
                }
            }
            Type::OpaqueVar(_) | Type::String(_) | Type::Optional(_) | Type::VarArray { .. } => None,
            Type::Array { element_type, size } => {
                let element_len = self.fixed_len_expr_inner(element_type, visiting)?;
                let size_value = if let Ok(size) = self.size_expr(size).parse::<u32>() {
                    FixedLen::Const(size)
                } else {
                    let size_expr = self.size_expr(size);
                    FixedLen::Expr(format!("size_as_u32({size_expr})?"))
                };
                Some(element_len.mul(size_value))
            }
            Type::Ident(name) => {
                let rust_name = type_name(name);
                if !visiting.insert(rust_name.clone()) {
                    return None;
                }

                let result = match self.type_info.definitions.get(&rust_name) {
                    Some(Definition::Typedef(typedef)) => {
                        self.fixed_len_expr_inner(&typedef.type_, visiting)
                    }
                    Some(Definition::Enum(_)) => Some(FixedLen::Const(4)),
                    Some(Definition::Struct(struct_)) => {
                        let mut total: Option<FixedLen> = None;
                        for member in &struct_.members {
                            let field_len = self.fixed_len_expr_inner(&member.type_, visiting)?;
                            total = Some(match total.take() {
                                Some(prefix) => prefix.add(field_len),
                                None => field_len,
                            });
                        }
                        total.or(Some(FixedLen::Const(0)))
                    }
                    Some(Definition::Union(union)) => {
                        if union.arms.iter().all(|arm| arm.type_.is_none()) {
                            Some(FixedLen::Const(4))
                        } else {
                            None
                        }
                    }
                    Some(Definition::Const(_)) | None => None,
                };

                visiting.remove(&rust_name);
                result
            }
        }
    }
}

fn strip_prefix(name: &str, prefix: &str) -> String {
    if let Some(stripped) = name.strip_prefix(prefix) {
        stripped.to_string()
    } else {
        name.to_string()
    }
}
