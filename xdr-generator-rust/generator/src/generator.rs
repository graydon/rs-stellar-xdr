use askama::Template;
use xdr_parser::ast::{
    Const, Definition, Enum, Struct, StructMember, Typedef, Union, UnionArm, XdrSpec,
};
use xdr_parser::lexer::IntBase;
use xdr_parser::types::{is_builtin_type, is_fixed_array, is_fixed_opaque, is_var_array, TypeInfo};

use crate::naming::{case_value, field_name, source_comment, type_name};
use crate::options::RustOptions;
use crate::output::{
    ConstOutput, CxxBridgeDefinition, CxxBridgeStruct, CxxBridgeStructMember,
    CxxBridgeTemplate, CxxBridgeTypedefNewtype, CxxBridgeUnion, CxxBridgeUnionArm,
    DefinitionOutput, EnumOutput, EnumStructMemberOutput, GeneratedTemplate, LazyAccessor,
    LazyContentValidation, LazyValidateFixedGroup, LazyValidateStep, LazyValidateVariable,
    LazyVarSkip, StructMemberOutput, StructOutput, TypeEnumOutput, TypedefAliasOutput,
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
                            let lazy_type = self.lazy_type_ref(&m.type_);
                            let scalar = self.is_lazy_scalar(&m.type_);
                            let cxx_scalar_type = if scalar {
                                self.cxx_scalar_type_name(&m.type_)
                            } else {
                                String::new()
                            };
                            CxxBridgeStructMember {
                                name: mname,
                                lazy_type,
                                lazy_is_scalar: scalar,
                                cxx_scalar_type,
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
                                    arm.type_.as_ref().map(|t| self.lazy_type_ref(t));
                                CxxBridgeUnionArm {
                                    case_name: cn,
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

    /// Map an XDR type to its CXX-compatible scalar type name.
    fn cxx_scalar_type_name(&self, type_: &xdr_parser::ast::Type) -> String {
        use xdr_parser::ast::Type;
        match type_ {
            Type::Int => "i32".to_string(),
            Type::UnsignedInt => "u32".to_string(),
            Type::Hyper => "i64".to_string(),
            Type::UnsignedHyper => "u64".to_string(),
            Type::Float => "f32".to_string(),
            Type::Double => "f64".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Ident(name) => {
                if let Some(def) = self.type_info.definitions.get(name.as_str()) {
                    if let Definition::Typedef(t) = def {
                        return self.cxx_scalar_type_name(&t.type_);
                    }
                    if let Definition::Enum(_) = def {
                        return "i32".to_string();
                    }
                }
                "i32".to_string() // fallback
            }
            _ => "i32".to_string(), // fallback
        }
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

        // Compute lazy per-field info up front.
        let mut field_infos: Vec<(String, String, bool, Option<u32>)> = Vec::new();
        for m in &s.members {
            let fname = field_name(&m.name);
            let ltype = self.lazy_type_ref(&m.type_);
            let scalar = self.is_lazy_scalar(&m.type_);
            let fixed = self.xdr_fixed_size(&m.type_);
            field_infos.push((fname, ltype, scalar, fixed));
        }
        let accessors = self.build_field_accessors(&field_infos);

        let members: Vec<StructMemberOutput> = s
            .members
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let mname = field_name(&m.name);
                let resolved = resolve_type(&m.type_, Some(&name), &self.type_info, custom_str);
                let (_, ref ltype, scalar, _) = field_infos[i];
                let acc = &accessors[i];
                StructMemberOutput {
                    name: mname,
                    type_ref: resolved.type_ref,
                    turbofish_type: resolved.turbofish_type,
                    serde_as_type: resolved.serde_as_type,
                    lazy_type: ltype.clone(),
                    lazy_is_scalar: scalar,
                    lazy_accessor: LazyAccessor {
                        initial_fixed: acc.initial_fixed,
                        var_skips: acc
                            .var_skips
                            .iter()
                            .map(|v| LazyVarSkip {
                                lazy_type: v.lazy_type.clone(),
                                post_fixed: v.post_fixed,
                            })
                            .collect(),
                    },
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
        let lazy_validate_steps = self.build_validate_steps(&s.members);

        StructOutput {
            name,
            source_comment: source_comment(&s.source, type_kind),
            has_default: !custom_default,
            is_custom_str: custom_str,
            members,
            member_names,
            lazy_name,
            lazy_fixed_size,
            lazy_validate_steps,
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
        let lazy_discriminant_type = self.lazy_type_ref(&u.discriminant.type_);
        let lazy_discriminant_is_enum = !discriminant_is_builtin;

        UnionOutput {
            name,
            source_comment: source_comment(&u.source, type_kind),
            has_default: !custom_default,
            is_custom_str: custom_str,
            discriminant_type,
            arms,
            lazy_name,
            lazy_discriminant_type,
            lazy_discriminant_is_enum,
        }
    }

    fn generate_typedef(&self, t: &Typedef) -> DefinitionOutput {
        let name = type_name(&t.name);

        if is_builtin_type(&t.type_) {
            let lazy_type = self.lazy_type_ref(&t.type_);
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
        let lazy_inner_type = self.lazy_type_ref(&t.type_);
        let lazy_fixed_size = self.xdr_fixed_size(&t.type_);
        let lazy_inner_is_scalar = self.is_lazy_scalar(&t.type_);

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
            lazy_inner_is_scalar,
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

                let lazy_type = arm.type_.as_ref().map(|t| self.lazy_type_ref(t));
                let case_value_i32 = match &case.value {
                    xdr_parser::ast::UnionCaseValue::Literal(n) => n.to_string(),
                    xdr_parser::ast::UnionCaseValue::Ident(ident) => {
                        // Look up the enum member value as an i32 literal.
                        self.resolve_enum_member_value(ident)
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| case_value_expr.clone())
                    }
                };

                UnionArmOutput {
                    case_name,
                    case_value: case_value_expr,
                    is_void: arm.type_.is_none(),
                    type_ref: resolved.as_ref().map(|r| r.type_ref.clone()),
                    turbofish_type: resolved.as_ref().map(|r| r.turbofish_type.clone()),
                    serde_as_type: resolved.and_then(|r| r.serde_as_type),
                    lazy_type,
                    case_value_i32,
                }
            })
            .collect()
    }

    // =====================================================================
    // Lazy type generation helpers
    // =====================================================================

    /// Look up the i32 value of an enum member by its XDR identifier name.
    fn resolve_enum_member_value(&self, ident: &str) -> Option<i32> {
        for def in self.type_info.definitions.values() {
            if let Definition::Enum(e) = def {
                for m in &e.members {
                    if m.name == ident {
                        return Some(m.value);
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

    // ---- Lazy type name mapping ----

    /// Return the Rust lazy type name for an XDR type.
    ///
    /// Scalars map to themselves. Complex types map to `Lazy*` wrappers.
    fn lazy_type_ref(&self, type_: &xdr_parser::ast::Type) -> String {
        use xdr_parser::ast::Type;
        match type_ {
            Type::Int => "i32".to_string(),
            Type::UnsignedInt => "u32".to_string(),
            Type::Hyper => "i64".to_string(),
            Type::UnsignedHyper => "u64".to_string(),
            Type::Float => "f32".to_string(),
            Type::Double => "f64".to_string(),
            Type::Bool => "bool".to_string(),
            Type::OpaqueFixed(size) => {
                format!("LazyOpaqueFixed::<{}>", self.resolve_size_str(size))
            }
            Type::OpaqueVar(max) => match max {
                Some(size) => format!("LazyBytesM::<{}>", self.resolve_size_str(size)),
                None => "LazyBytesM".to_string(),
            },
            Type::String(max) => match max {
                Some(size) => format!("LazyStringM::<{}>", self.resolve_size_str(size)),
                None => "LazyStringM".to_string(),
            },
            Type::Ident(ident_name) => {
                let resolved_name = type_name(ident_name);
                match self.type_info.definitions.get(&resolved_name) {
                    Some(Definition::Enum(_)) => {
                        // Enums are scalars — reference the parent module's enum.
                        format!("super::{resolved_name}")
                    }
                    Some(Definition::Typedef(t)) if is_builtin_type(&t.type_) => {
                        // Typedef alias to builtin — use the scalar type.
                        self.lazy_type_ref(&t.type_)
                    }
                    _ => {
                        // Struct, union, typedef wrapping complex → Lazy wrapper.
                        format!("Lazy{resolved_name}")
                    }
                }
            }
            Type::Optional(inner) => {
                let inner_lazy = self.lazy_type_ref(inner);
                format!("LazyOption::<{inner_lazy}>")
            }
            Type::Array { element_type, size } => {
                let elem_lazy = self.lazy_type_ref(element_type);
                format!(
                    "LazyFixedArray::<{elem_lazy}, {}>",
                    self.resolve_size_str(size)
                )
            }
            Type::VarArray {
                element_type,
                max_size,
            } => {
                let elem_lazy = self.lazy_type_ref(element_type);
                match max_size {
                    Some(size) => {
                        format!(
                            "LazyVecM::<{elem_lazy}, {}>",
                            self.resolve_size_str(size)
                        )
                    }
                    None => format!("LazyVecM::<{elem_lazy}>"),
                }
            }
        }
    }

    /// Whether a type maps to a scalar (non-handle) lazy type.
    fn is_lazy_scalar(&self, type_: &xdr_parser::ast::Type) -> bool {
        use xdr_parser::ast::Type;
        match type_ {
            Type::Int | Type::UnsignedInt | Type::Hyper | Type::UnsignedHyper | Type::Float
            | Type::Double | Type::Bool => true,
            Type::Ident(ident_name) => {
                let resolved_name = type_name(ident_name);
                match self.type_info.definitions.get(&resolved_name) {
                    Some(Definition::Enum(_)) => true,
                    Some(Definition::Typedef(t)) if is_builtin_type(&t.type_) => {
                        self.is_lazy_scalar(&t.type_)
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    /// Whether a type needs content validation beyond a size check.
    fn needs_content_validation(&self, type_: &xdr_parser::ast::Type) -> bool {
        use xdr_parser::ast::Type;
        match type_ {
            Type::Int | Type::UnsignedInt | Type::Hyper | Type::UnsignedHyper | Type::Float
            | Type::Double => false,
            Type::Bool => true,
            Type::OpaqueFixed(_) => true, // padding check
            Type::OpaqueVar(_) | Type::String(_) => true, // length + padding
            Type::Ident(ident_name) => {
                let resolved_name = type_name(ident_name);
                match self.type_info.definitions.get(&resolved_name) {
                    Some(Definition::Enum(_)) => true,
                    Some(Definition::Struct(_)) => true,
                    Some(Definition::Union(_)) => true,
                    Some(Definition::Typedef(t)) => self.needs_content_validation(&t.type_),
                    _ => true,
                }
            }
            Type::Optional(_) => true,
            Type::Array { element_type, .. } => self.needs_content_validation(element_type),
            Type::VarArray { .. } => true,
        }
    }

    // ---- Validation step builder ----

    fn build_validate_steps(&self, members: &[StructMember]) -> Vec<LazyValidateStep> {
        let mut steps: Vec<LazyValidateStep> = Vec::new();

        // Accumulate consecutive fixed-size fields into groups.
        let mut current_fixed: u32 = 0;
        let mut current_validations: Vec<LazyContentValidation> = Vec::new();

        for m in members {
            let fixed = self.xdr_fixed_size(&m.type_);
            if let Some(fsize) = fixed {
                // Fixed-size field — accumulate.
                if self.needs_content_validation(&m.type_) {
                    current_validations.push(LazyContentValidation {
                        offset: current_fixed,
                        lazy_type: self.lazy_type_ref(&m.type_),
                    });
                }
                current_fixed += fsize;
            } else {
                // Variable-size field — flush any accumulated fixed group first.
                if current_fixed > 0 || !current_validations.is_empty() {
                    steps.push(LazyValidateStep::FixedGroup(LazyValidateFixedGroup {
                        total_fixed: current_fixed,
                        content_validations: std::mem::take(&mut current_validations),
                    }));
                    current_fixed = 0;
                }
                steps.push(LazyValidateStep::Variable(LazyValidateVariable {
                    lazy_type: self.lazy_type_ref(&m.type_),
                }));
            }
        }

        // Flush trailing fixed group.
        if current_fixed > 0 || !current_validations.is_empty() {
            steps.push(LazyValidateStep::FixedGroup(LazyValidateFixedGroup {
                total_fixed: current_fixed,
                content_validations: current_validations,
            }));
        }

        steps
    }

    // ---- Field accessor builder ----

    fn build_field_accessors(
        &self,
        field_infos: &[(String, String, bool, Option<u32>)],
    ) -> Vec<LazyAccessor> {
        let mut result = Vec::new();

        for i in 0..field_infos.len() {
            let mut initial_fixed: u32 = 0;
            let mut var_skips: Vec<LazyVarSkip> = Vec::new();
            let mut in_var = false;

            for j in 0..i {
                let (_, ref prev_lazy_type, _, prev_fixed) = field_infos[j];
                if let Some(fsize) = prev_fixed {
                    if in_var {
                        var_skips.last_mut().unwrap().post_fixed += fsize;
                    } else {
                        initial_fixed += fsize;
                    }
                } else {
                    var_skips.push(LazyVarSkip {
                        lazy_type: prev_lazy_type.clone(),
                        post_fixed: 0,
                    });
                    in_var = true;
                }
            }

            result.push(LazyAccessor {
                initial_fixed,
                var_skips,
            });
        }

        result
    }
}
