use askama::Template;
use xdr_parser::ast::{
    Const, Definition, Enum, Struct, StructMember, Typedef, Union, UnionArm, XdrSpec,
};
use xdr_parser::lexer::IntBase;
use xdr_parser::types::{is_builtin_type, is_fixed_array, is_fixed_opaque, is_var_array, TypeInfo};

use crate::naming::{case_value, field_name, source_comment, type_name};
use crate::options::RustOptions;
use crate::output::{
    ConstOutput, CxxBridgeTemplate, DefinitionOutput, EnumOutput, EnumStructMemberOutput,
    GeneratedTemplate, StructMemberOutput, StructOutput, TypeEnumOutput, TypedefAliasOutput,
    TypedefNewtypeOutput, UnionArmOutput, UnionOutput,
};
use crate::lazy_codegen::LazyCodegen;
use crate::output::CxxBridgeTypeOutput;
use crate::types::{base_type_ref, resolve_type, size_to_string, type_ref};
use std::collections::HashSet;

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
        let template = self.generate(spec, header);
        let rendered = template.render()?;
        std::fs::write(output, rendered)?;
        Ok(())
    }

    pub fn generate_cxx_bridge_to_file(
        &self,
        spec: &XdrSpec,
        output: &std::path::PathBuf,
        bridge_types: Option<&HashSet<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let template = self.generate_cxx_bridge(spec, bridge_types)?;
        let rendered = template.render()?;
        std::fs::write(output, rendered)?;
        Ok(())
    }

    /// Generate output for the entire spec.
    pub fn generate(&self, spec: &XdrSpec, header: &str) -> GeneratedTemplate {
        let xdr_files_sha256: Vec<(String, String)> = spec
            .files
            .iter()
            .map(|f| (f.name.clone(), f.sha256.clone()))
            .collect();

        let lazy_codegen = LazyCodegen::new(&self.type_info);
        let mut definitions: Vec<DefinitionOutput> = Vec::new();

        for def in spec.all_definitions() {
            let output = self.generate_definition(def, &lazy_codegen);
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
        }
    }

    pub fn generate_cxx_bridge(
        &self,
        spec: &XdrSpec,
        bridge_types: Option<&HashSet<String>>,
    ) -> Result<CxxBridgeTemplate, Box<dyn std::error::Error>> {
        let available: Vec<String> = spec
            .all_definitions()
            .filter(|def| !matches!(def, Definition::Const(_)))
            .map(|def| type_name(def.name()))
            .collect();

        if let Some(requested) = bridge_types {
            let available_set: HashSet<String> = available.iter().cloned().collect();
            let unknown: Vec<String> = requested
                .iter()
                .filter(|name| !available_set.contains(*name))
                .cloned()
                .collect();
            if !unknown.is_empty() {
                return Err(format!(
                    "unknown bridge types: {}",
                    unknown.join(", ")
                )
                .into());
            }
        }

        let bridge_types = available
            .into_iter()
            .filter(|name| bridge_types.map(|requested| requested.contains(name)).unwrap_or(true))
            .map(|name| CxxBridgeTypeOutput { name })
            .collect();

        Ok(CxxBridgeTemplate { bridge_types })
    }

    fn generate_definition(&self, def: &Definition, lazy_codegen: &LazyCodegen<'_>) -> DefinitionOutput {
        match def {
            Definition::Struct(s) => DefinitionOutput::Struct(self.generate_struct(s, lazy_codegen)),
            Definition::Enum(e) => DefinitionOutput::Enum(self.generate_enum(e, lazy_codegen)),
            Definition::Union(u) => DefinitionOutput::Union(self.generate_union(u, lazy_codegen)),
            Definition::Typedef(t) => self.generate_typedef(t, lazy_codegen),
            Definition::Const(c) => DefinitionOutput::Const(self.generate_const(c)),
        }
    }

    fn generate_struct(&self, s: &Struct, lazy_codegen: &LazyCodegen<'_>) -> StructOutput {
        let name = type_name(&s.name);
        let custom_default = self.options.custom_default_impl.contains(&name);
        let custom_str = self.options.custom_str_impl.contains(&name);

        let members: Vec<StructMemberOutput> = s
            .members
            .iter()
            .enumerate()
            .map(|(index, m)| self.generate_member(m, &s.members, index, &name, custom_str, lazy_codegen))
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
        StructOutput {
            name,
            source_comment: source_comment(&s.source, type_kind),
            has_default: !custom_default,
            is_custom_str: custom_str,
            members,
            lazy_len_steps: lazy_codegen.scan_steps(&s.members, s.members.len()),
            member_names,
        }
    }

    fn generate_enum(&self, e: &Enum, lazy_codegen: &LazyCodegen<'_>) -> EnumOutput {
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
            lazy_value: lazy_codegen.value_output(&xdr_parser::ast::Type::Ident(e.name.clone())),
        }
    }

    fn generate_union(&self, u: &Union, lazy_codegen: &LazyCodegen<'_>) -> UnionOutput {
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
            .enumerate()
            .flat_map(|arm| {
                self.generate_union_arm(
                    u,
                    arm.0,
                    arm.1,
                    &name,
                    &discriminant_type,
                    discriminant_is_builtin,
                    &discriminant_prefix,
                    custom_str,
                    lazy_codegen,
                )
            })
            .collect();

        let type_kind = if u.is_nested { "NestedUnion" } else { "Union" };

        UnionOutput {
            name,
            source_comment: source_comment(&u.source, type_kind),
            has_default: !custom_default,
            is_custom_str: custom_str,
            discriminant_type,
            arms,
            lazy_discriminant: lazy_codegen.value_output(&u.discriminant.type_),
        }
    }

    fn generate_typedef(&self, t: &Typedef, lazy_codegen: &LazyCodegen<'_>) -> DefinitionOutput {
        let name = type_name(&t.name);

        if is_builtin_type(&t.type_) {
            return DefinitionOutput::TypedefAlias(TypedefAliasOutput {
                name,
                source_comment: source_comment(&t.source, "Typedef"),
                type_ref: base_type_ref(&t.type_, None),
                lazy_type: lazy_codegen.type_output(&t.type_),
                lazy_value: lazy_codegen.value_output(&t.type_),
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
            lazy_inner_type: lazy_codegen.type_output(&t.type_),
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

    fn generate_member(
        &self,
        m: &StructMember,
        members: &[StructMember],
        index: usize,
        parent: &str,
        custom_str: bool,
        lazy_codegen: &LazyCodegen<'_>,
    ) -> StructMemberOutput {
        let name = field_name(&m.name);
        let resolved = resolve_type(&m.type_, Some(parent), &self.type_info, custom_str);

        StructMemberOutput {
            name,
            type_ref: resolved.type_ref,
            turbofish_type: resolved.turbofish_type,
            serde_as_type: resolved.serde_as_type,
            lazy_type: lazy_codegen.type_output(&m.type_),
            lazy_scan_steps: lazy_codegen.scan_steps(members, index),
        }
    }

    fn generate_union_arm(
        &self,
        union: &Union,
        arm_index: usize,
        arm: &UnionArm,
        parent: &str,
        discriminant_type: &str,
        discriminant_is_builtin: bool,
        discriminant_prefix: &str,
        custom_str: bool,
        lazy_codegen: &LazyCodegen<'_>,
    ) -> Vec<UnionArmOutput> {
        arm.cases
            .iter()
            .enumerate()
            .map(|case| {
                let (case_name, case_value_expr) = case_value(
                    discriminant_type,
                    discriminant_is_builtin,
                    &case.1.value,
                    discriminant_prefix,
                );

                let resolved = arm
                    .type_
                    .as_ref()
                    .map(|t| resolve_type(t, Some(parent), &self.type_info, custom_str));

                UnionArmOutput {
                    case_name,
                    case_value: case_value_expr,
                    is_void: arm.type_.is_none(),
                    type_ref: resolved.as_ref().map(|r| r.type_ref.clone()),
                    turbofish_type: resolved.as_ref().map(|r| r.turbofish_type.clone()),
                    serde_as_type: resolved.and_then(|r| r.serde_as_type),
                    lazy_method_name: lazy_codegen.union_arm_method_name(union, arm_index, case.0),
                    lazy_case: lazy_codegen.union_case_output(union, arm_index, case.0),
                    lazy_type: arm.type_.as_ref().map(|ty| lazy_codegen.type_output(ty)),
                }
            })
            .collect()
    }
}
