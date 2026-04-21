use askama::Template;
use xdr_parser::ast::{
    Const, Definition, Enum, Struct, StructMember, Typedef, Union, UnionArm, XdrSpec,
};
use xdr_parser::lexer::IntBase;
use xdr_parser::types::{is_builtin_type, is_fixed_array, is_fixed_opaque, is_var_array, TypeInfo};

use crate::naming::{case_value, field_name, source_comment, type_name};
use crate::options::RustOptions;
use crate::output::{
    ConstOutput, CxxBridgeDefinition, CxxBridgeStruct, CxxBridgeStructMember, CxxBridgeTemplate,
    CxxBridgeTypedefNewtype, CxxBridgeUnion, CxxBridgeUnionArm, DefinitionOutput, EnumOutput,
    EnumStructMemberOutput, GeneratedTemplate, LazyScanStepOutput, LazyTypeOutput,
    LazyValueOutput, StructMemberOutput, StructOutput, TypeEnumOutput, TypedefAliasOutput,
    TypedefNewtypeOutput, UnionArmOutput, UnionOutput,
};
use crate::types::{base_type_ref, resolve_type, size_to_string, type_ref};

pub struct RustGenerator {
    options: RustOptions,
    type_info: TypeInfo,
}

impl RustGenerator {
    pub fn new(spec: &XdrSpec, options: RustOptions) -> Self {
        let type_info = TypeInfo::build(spec, &type_name);
        Self { options, type_info }
    }

    /// Generate Rust code from the spec and write it to the output file.
    pub fn generate_to_file(
        &self,
        spec: &XdrSpec,
        output: &std::path::PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let header = include_str!("../header.rs");
        let lazy_header = include_str!("../lazy_header.rs");
        let template = self.generate(spec, header, lazy_header);
        let rendered = template.render()?;
        std::fs::write(output, rendered)?;
        Ok(())
    }

    /// Generate a standalone CXX bridge module file for the given types.
    pub fn generate_cxx_bridge_to_file(
        &self,
        spec: &XdrSpec,
        output: &std::path::PathBuf,
        bridge_types: &std::collections::HashSet<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let template = self.generate_cxx_bridge(spec, bridge_types);
        let rendered = template.render()?;
        std::fs::write(output, rendered)?;
        Ok(())
    }

    /// Build the CXX bridge template for the requested types.
    fn generate_cxx_bridge(
        &self,
        spec: &XdrSpec,
        bridge_types: &std::collections::HashSet<String>,
    ) -> CxxBridgeTemplate {
        let mut definitions = Vec::new();

        for def in spec.all_definitions() {
            let name = match def {
                Definition::Struct(s) => &s.name,
                Definition::Union(u) => &u.name,
                Definition::Typedef(t) => &t.name,
                _ => continue,
            };
            let rust_name = type_name(name);
            if !bridge_types.contains(&rust_name) {
                continue;
            }

            match def {
                Definition::Struct(s) => {
                    let members = s
                        .members
                        .iter()
                        .map(|m| {
                            let mname = field_name(&m.name);
                            let lazy_type = self.lazy_type_output(&m.type_);
                            CxxBridgeStructMember {
                                name: mname,
                                lazy_type,
                            }
                        })
                        .collect();
                    definitions.push(CxxBridgeDefinition::Struct(CxxBridgeStruct {
                        lazy_name: format!("Lazy{rust_name}"),
                        members,
                    }));
                }
                Definition::Union(u) => {
                    let discriminant_type_str =
                        type_ref(&u.discriminant.type_, None, &self.type_info);
                    let discriminant_is_builtin = is_builtin_type(&u.discriminant.type_)
                        || matches!(&u.discriminant.type_, xdr_parser::ast::Type::Ident(n) if {
                            self.type_info.definitions.get(&type_name(n))
                                .map(|d| matches!(d, Definition::Typedef(t) if is_builtin_type(&t.type_)))
                                .unwrap_or(false)
                        });
                    let discriminant_prefix = if !discriminant_is_builtin {
                        self.type_info
                            .discriminant_enum(&u.discriminant.type_)
                            .map(|e| e.member_prefix.clone())
                            .unwrap_or_default()
                    } else {
                        String::new()
                    };

                    let arms = u
                        .arms
                        .iter()
                        .flat_map(|arm| {
                            let dt = &discriminant_type_str;
                            let dp = &discriminant_prefix;
                            arm.cases.iter().map(move |case| {
                                let (cn, _) = case_value(
                                    dt,
                                    discriminant_is_builtin,
                                    &case.value,
                                    dp,
                                );
                                let lazy_type =
                                    arm.type_.as_ref().map(|t| self.lazy_type_output(t));
                                CxxBridgeUnionArm {
                                    lazy_method_name: format!("as_{}", field_name(&cn)),
                                    is_void: arm.type_.is_none(),
                                    lazy_type,
                                }
                            })
                        })
                        .collect();
                    definitions.push(CxxBridgeDefinition::Union(CxxBridgeUnion {
                        lazy_name: format!("Lazy{rust_name}"),
                        arms,
                    }));
                }
                Definition::Typedef(_) => {
                    definitions.push(CxxBridgeDefinition::TypedefNewtype(
                        CxxBridgeTypedefNewtype {
                            lazy_name: format!("Lazy{rust_name}"),
                        },
                    ));
                }
                _ => {}
            }
        }

        CxxBridgeTemplate { definitions }
    }

    /// Generate output for the entire spec.
    pub fn generate(&self, spec: &XdrSpec, header: &str, lazy_header: &str) -> GeneratedTemplate {
        let xdr_files_sha256: Vec<(String, String)> = spec
            .files
            .iter()
            .map(|f| (f.name.clone(), f.sha256.clone()))
            .collect();

        let mut definitions: Vec<DefinitionOutput> = Vec::new();

        for def in spec.all_definitions() {
            let output = self.generate_definition(def);
            definitions.push(output);
        }

        let types: Vec<String> = spec
            .type_names_parent_first()
            .iter()
            .map(|name| type_name(name))
            .collect();

        GeneratedTemplate {
            xdr_files_sha256,
            header: header.to_string(),
            definitions,
            type_variant_enum: TypeEnumOutput { types },
            lazy_header: lazy_header.to_string(),
        }
    }

    fn generate_definition(&self, def: &Definition) -> DefinitionOutput {
        match def {
            Definition::Struct(s) => DefinitionOutput::Struct(self.generate_struct(s)),
            Definition::Enum(e) => DefinitionOutput::Enum(self.generate_enum(e)),
            Definition::Union(u) => DefinitionOutput::Union(self.generate_union(u)),
            Definition::Typedef(t) => self.generate_typedef(t),
            Definition::Const(c) => DefinitionOutput::Const(self.generate_const(c)),
        }
    }

    fn generate_struct(&self, s: &Struct) -> StructOutput {
        let name = type_name(&s.name);
        let custom_default = self.options.custom_default_impl.contains(&name);
        let custom_str = self.options.custom_str_impl.contains(&name);

        let members: Vec<StructMemberOutput> = s
            .members
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let mname = field_name(&m.name);
                let resolved = resolve_type(&m.type_, Some(&name), &self.type_info, custom_str);
                StructMemberOutput {
                    name: mname,
                    type_ref: resolved.type_ref,
                    turbofish_type: resolved.turbofish_type,
                    serde_as_type: resolved.serde_as_type,
                    lazy_type: self.lazy_type_output(&m.type_),
                    lazy_scan_steps: self
                        .build_scan_steps(s.members[..i].iter().map(|member| &member.type_)),
                }
            })
            .collect();

        let member_names: String = members
            .iter()
            .map(|m| m.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        let type_kind = if s.is_nested {
            "NestedStruct"
        } else {
            "Struct"
        };

        let lazy_name = format!("Lazy{name}");
        let lazy_fixed_size = self.compute_total_fixed_size(&s.members);
        let lazy_len_steps = self.build_scan_steps(s.members.iter().map(|member| &member.type_));

        StructOutput {
            name,
            source_comment: source_comment(&s.source, type_kind),
            has_default: !custom_default,
            is_custom_str: custom_str,
            members,
            member_names,
            lazy_name,
            lazy_fixed_size,
            lazy_len_steps,
        }
    }

    fn generate_enum(&self, e: &Enum) -> EnumOutput {
        let name = type_name(&e.name);
        let custom_default = self.options.custom_default_impl.contains(&name);
        let custom_str = self.options.custom_str_impl.contains(&name);

        let members: Vec<EnumStructMemberOutput> = e
            .members
            .iter()
            .enumerate()
            .map(|(i, m)| EnumStructMemberOutput {
                name: type_name(&m.stripped_name),
                value: m.value,
                is_first: i == 0,
            })
            .collect();

        EnumOutput {
            name,
            source_comment: source_comment(&e.source, "Enum"),
            has_default: !custom_default,
            is_custom_str: custom_str,
            members,
        }
    }

    fn generate_union(&self, u: &Union) -> UnionOutput {
        let name = type_name(&u.name);
        let custom_default = self.options.custom_default_impl.contains(&name);
        let custom_str = self.options.custom_str_impl.contains(&name);

        let discriminant_type = type_ref(&u.discriminant.type_, None, &self.type_info);
        let discriminant_is_builtin = is_builtin_type(&u.discriminant.type_)
            || matches!(&u.discriminant.type_, xdr_parser::ast::Type::Ident(n) if {
                self.type_info.definitions.get(&type_name(n))
                    .map(|d| matches!(d, Definition::Typedef(t) if is_builtin_type(&t.type_)))
                    .unwrap_or(false)
            });

        let discriminant_prefix = if !discriminant_is_builtin {
            self.type_info
                .discriminant_enum(&u.discriminant.type_)
                .map(|e| e.member_prefix.clone())
                .unwrap_or_default()
        } else {
            String::new()
        };

        let arms: Vec<UnionArmOutput> = u
            .arms
            .iter()
            .flat_map(|arm| {
                self.generate_union_arm(
                    arm,
                    &name,
                    &discriminant_type,
                    discriminant_is_builtin,
                    &discriminant_prefix,
                    custom_str,
                )
            })
            .collect();

        let type_kind = if u.is_nested { "NestedUnion" } else { "Union" };

        let lazy_name = format!("Lazy{name}");
        let lazy_discriminant = self.lazy_value_output(&u.discriminant.type_);

        UnionOutput {
            name,
            source_comment: source_comment(&u.source, type_kind),
            has_default: !custom_default,
            is_custom_str: custom_str,
            discriminant_type,
            arms,
            lazy_name,
            lazy_discriminant,
        }
    }

    fn generate_typedef(&self, t: &Typedef) -> DefinitionOutput {
        let name = type_name(&t.name);

        if is_builtin_type(&t.type_) {
            let lazy_type = self.lazy_type_output(&t.type_);
            return DefinitionOutput::TypedefAlias(TypedefAliasOutput {
                name,
                source_comment: source_comment(&t.source, "Typedef"),
                type_ref: base_type_ref(&t.type_, None),
                lazy_type,
            });
        }

        let custom_default = self.options.custom_default_impl.contains(&name);
        let custom_str = self.options.custom_str_impl.contains(&name);
        let no_display_fromstr = self.options.no_display_fromstr.contains(&name);
        let is_fixed_opaque_type = is_fixed_opaque(&t.type_);
        let is_fixed_array_type = is_fixed_array(&t.type_);
        let is_var_array_type = is_var_array(&t.type_);

        let resolved = resolve_type(&t.type_, None, &self.type_info, custom_str);

        let size = match &t.type_ {
            xdr_parser::ast::Type::OpaqueFixed(s)
            | xdr_parser::ast::Type::Array { size: s, .. } => Some(size_to_string(s)),
            _ => None,
        };

        let lazy_name = format!("Lazy{name}");
        let lazy_inner_type = self.lazy_type_output(&t.type_);
        let lazy_fixed_size = self.xdr_fixed_size(&t.type_);

        DefinitionOutput::TypedefNewtype(TypedefNewtypeOutput {
            name,
            source_comment: source_comment(&t.source, "Typedef"),
            has_default: !custom_default,
            is_var_array: is_var_array_type,
            is_fixed_opaque: is_fixed_opaque_type,
            is_fixed_array: is_fixed_array_type,
            is_custom_str: custom_str,
            type_ref: resolved.type_ref,
            turbofish_type: resolved.turbofish_type,
            serde_as_type: resolved.serde_as_type,
            element_type: resolved.element_type,
            size,
            custom_debug: is_fixed_opaque_type,
            custom_display_fromstr: is_fixed_opaque_type && !custom_str && !no_display_fromstr,
            custom_schemars: is_fixed_opaque_type && !custom_str && !no_display_fromstr,
            lazy_name,
            lazy_inner_type,
            lazy_fixed_size,
        })
    }

    fn generate_const(&self, c: &Const) -> ConstOutput {
        let value_str = match c.base {
            IntBase::Hexadecimal => format!("0x{:X}", c.value),
            IntBase::Decimal => c.value.to_string(),
        };
        ConstOutput {
            name: field_name(&c.name).to_uppercase(),
            doc_name: type_name(&c.name),
            source_comment: source_comment(&c.source, "Const"),
            value_str,
        }
    }


    fn generate_union_arm(
        &self,
        arm: &UnionArm,
        parent: &str,
        discriminant_type: &str,
        discriminant_is_builtin: bool,
        discriminant_prefix: &str,
        custom_str: bool,
    ) -> Vec<UnionArmOutput> {
        arm.cases
            .iter()
            .map(|case| {
                let (case_name, case_value_expr) = case_value(
                    discriminant_type,
                    discriminant_is_builtin,
                    &case.value,
                    discriminant_prefix,
                );

                let resolved = arm
                    .type_
                    .as_ref()
                    .map(|t| resolve_type(t, Some(parent), &self.type_info, custom_str));

                let lazy_type = arm.type_.as_ref().map(|t| self.lazy_type_output(t));
                let case_value_i32 = match &case.value {
                    xdr_parser::ast::UnionCaseValue::Literal(n) => n.to_string(),
                    xdr_parser::ast::UnionCaseValue::Ident(ident) => self
                        .resolve_enum_member_value(ident)
                        .map(|value| value.to_string())
                        .unwrap_or_else(|| case_value_expr.clone()),
                };

                UnionArmOutput {
                    lazy_method_name: format!("as_{}", field_name(&case_name)),
                    case_name,
                    case_value: case_value_expr,
                    case_value_i32,
                    is_void: arm.type_.is_none(),
                    type_ref: resolved.as_ref().map(|r| r.type_ref.clone()),
                    turbofish_type: resolved.as_ref().map(|r| r.turbofish_type.clone()),
                    serde_as_type: resolved.and_then(|r| r.serde_as_type),
                    lazy_type,
                }
            })
            .collect()
    }

    // =====================================================================
    // Lazy type generation helpers
    // =====================================================================

    fn resolve_enum_member_value(&self, ident: &str) -> Option<i32> {
        for def in self.type_info.definitions.values() {
            if let Definition::Enum(e) = def {
                for member in &e.members {
                    if member.name == ident {
                        return Some(member.value);
                    }
                }
            }
        }
        None
    }

    // ---- XDR wire size computation ----

    /// Compute the fixed XDR wire size of a type, or None if variable-length.
    fn xdr_fixed_size(&self, type_: &xdr_parser::ast::Type) -> Option<u32> {
        use xdr_parser::ast::Type;
        match type_ {
            Type::Int | Type::UnsignedInt | Type::Float | Type::Bool => Some(4),
            Type::Hyper | Type::UnsignedHyper | Type::Double => Some(8),
            Type::OpaqueFixed(size) => {
                let n = self.resolve_size_literal(size)?;
                Some((n + 3) & !3) // pad to 4
            }
            Type::Array { element_type, size } => {
                let n = self.resolve_size_literal(size)?;
                let elem_size = self.xdr_fixed_size(element_type)?;
                n.checked_mul(elem_size)
            }
            Type::Ident(ident_name) => {
                let resolved_name = type_name(ident_name);
                match self.type_info.definitions.get(&resolved_name) {
                    Some(Definition::Enum(_)) => Some(4),
                    Some(Definition::Struct(s)) => self.compute_total_fixed_size(&s.members),
                    Some(Definition::Typedef(t)) => self.xdr_fixed_size(&t.type_),
                    Some(Definition::Union(_)) => None,
                    _ => None,
                }
            }
            Type::OpaqueVar(_)
            | Type::String(_)
            | Type::VarArray { .. }
            | Type::Optional(_) => None,
        }
    }

    /// Resolve a Size to a u32 literal, if possible.
    fn resolve_size_literal(&self, size: &xdr_parser::ast::Size) -> Option<u32> {
        match size {
            xdr_parser::ast::Size::Literal(n) => Some(*n),
            xdr_parser::ast::Size::Named(name) => {
                self.type_info
                    .const_values
                    .get(name)
                    .and_then(|&v| u32::try_from(v).ok())
            }
        }
    }

    fn resolve_size_str(&self, size: &xdr_parser::ast::Size) -> String {
        self.type_info.size_to_literal(size)
    }

    /// Total fixed size of a struct's members, or None if any is variable.
    fn compute_total_fixed_size(&self, members: &[StructMember]) -> Option<u32> {
        let mut total: u32 = 0;
        for m in members {
            let field_size = self.xdr_fixed_size(&m.type_)?;
            total = total.checked_add(field_size)?;
        }
        Some(total)
    }

    // ---- Lazy type mapping ----

    fn lazy_type_output(&self, type_: &xdr_parser::ast::Type) -> LazyTypeOutput {
        use xdr_parser::ast::Type;
        match type_ {
            Type::Int => LazyTypeOutput::I32,
            Type::UnsignedInt => LazyTypeOutput::U32,
            Type::Hyper => LazyTypeOutput::I64,
            Type::UnsignedHyper => LazyTypeOutput::U64,
            Type::Float => LazyTypeOutput::F32,
            Type::Double => LazyTypeOutput::F64,
            Type::Bool => LazyTypeOutput::Bool,
            Type::OpaqueFixed(size) => LazyTypeOutput::FixedOpaque {
                size: self.resolve_size_str(size),
            },
            Type::OpaqueVar(max) => LazyTypeOutput::VarOpaque {
                max_size: max.as_ref().map(|size| self.resolve_size_str(size)),
            },
            Type::String(max) => LazyTypeOutput::String {
                max_size: max.as_ref().map(|size| self.resolve_size_str(size)),
            },
            Type::Ident(ident_name) => {
                let resolved_name = type_name(ident_name);
                match self.type_info.definitions.get(&resolved_name) {
                    Some(Definition::Enum(_)) => LazyTypeOutput::NamedEnum { name: resolved_name },
                    Some(Definition::Typedef(t)) if is_builtin_type(&t.type_) => {
                        self.lazy_type_output(&t.type_)
                    }
                    _ => LazyTypeOutput::NamedLazy { name: resolved_name },
                }
            }
            Type::Optional(inner) => LazyTypeOutput::Optional {
                inner: Box::new(self.lazy_type_output(inner)),
            },
            Type::Array { element_type, size } => LazyTypeOutput::Array {
                element: Box::new(self.lazy_type_output(element_type)),
                size: self.resolve_size_str(size),
            },
            Type::VarArray {
                element_type,
                max_size,
            } => LazyTypeOutput::VarArray {
                element: Box::new(self.lazy_type_output(element_type)),
                max_size: max_size.as_ref().map(|size| self.resolve_size_str(size)),
            },
        }
    }

    fn lazy_value_output(&self, type_: &xdr_parser::ast::Type) -> LazyValueOutput {
        use xdr_parser::ast::Type;
        match type_ {
            Type::Int => LazyValueOutput::I32,
            Type::UnsignedInt => LazyValueOutput::U32,
            Type::Hyper => LazyValueOutput::I64,
            Type::UnsignedHyper => LazyValueOutput::U64,
            Type::Bool => LazyValueOutput::Bool,
            Type::Ident(ident_name) => {
                let resolved_name = type_name(ident_name);
                match self.type_info.definitions.get(&resolved_name) {
                    Some(Definition::Enum(_)) => LazyValueOutput::NamedEnum { name: resolved_name },
                    Some(Definition::Typedef(t)) if is_builtin_type(&t.type_) => {
                        self.lazy_value_output(&t.type_)
                    }
                    _ => panic!("non-scalar value type used in lazy generation"),
                }
            }
            _ => panic!("unsupported scalar value type in lazy generation"),
        }
    }

    fn build_scan_steps<'a>(
        &self,
        types: impl Iterator<Item = &'a xdr_parser::ast::Type>,
    ) -> Vec<LazyScanStepOutput> {
        let mut result = Vec::new();
        let mut current_fixed: u32 = 0;

        for type_ in types {
            if let Some(fixed_size) = self.xdr_fixed_size(type_) {
                current_fixed = current_fixed
                    .checked_add(fixed_size)
                    .expect("fixed-size scan plan overflow");
                continue;
            }

            if current_fixed > 0 {
                result.push(LazyScanStepOutput::Fixed {
                    len_expr: current_fixed.to_string(),
                });
                current_fixed = 0;
            }

            result.push(LazyScanStepOutput::Variable {
                type_output: self.lazy_type_output(type_),
            });
        }

        if current_fixed > 0 {
            result.push(LazyScanStepOutput::Fixed {
                len_expr: current_fixed.to_string(),
            });
        }

        result
    }
}
