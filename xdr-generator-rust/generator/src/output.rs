use askama::Template;

#[derive(Template)]
#[template(path = "generated.rs.jinja", escape = "none")]
pub struct GeneratedTemplate {
    pub xdr_files_sha256: Vec<(String, String)>,
    pub header: String,
    pub definitions: Vec<DefinitionOutput>,
    pub type_variant_enum: TypeEnumOutput,
    pub lazy_header: String,
}

#[derive(Template)]
#[template(path = "cxx_bridge.rs.jinja", escape = "none")]
pub struct CxxBridgeTemplate {
    pub definitions: Vec<CxxBridgeDefinition>,
}

pub enum CxxBridgeDefinition {
    Struct(CxxBridgeStruct),
    Union(CxxBridgeUnion),
    TypedefNewtype(CxxBridgeTypedefNewtype),
}

pub struct CxxBridgeStruct {
    pub lazy_name: String,
    pub members: Vec<CxxBridgeStructMember>,
}

pub struct CxxBridgeStructMember {
    pub name: String,
    pub lazy_type: String,
    pub lazy_is_scalar: bool,
    /// CXX-compatible scalar type name (e.g. `i32`, `u64`, `bool`).
    pub cxx_scalar_type: String,
}

pub struct CxxBridgeUnion {
    pub lazy_name: String,
    pub arms: Vec<CxxBridgeUnionArm>,
}

pub struct CxxBridgeUnionArm {
    pub case_name: String,
    pub is_void: bool,
    pub lazy_type: Option<String>,
}

pub struct CxxBridgeTypedefNewtype {
    pub lazy_name: String,
}

pub enum DefinitionOutput {
    Struct(StructOutput),
    Enum(EnumOutput),
    Union(UnionOutput),
    TypedefAlias(TypedefAliasOutput),
    TypedefNewtype(TypedefNewtypeOutput),
    Const(ConstOutput),
}

pub struct StructOutput {
    pub name: String,
    pub source_comment: String,
    pub has_default: bool,
    pub is_custom_str: bool,
    pub members: Vec<StructMemberOutput>,
    pub member_names: String,
    // Lazy fields
    pub lazy_name: String,
    pub lazy_fixed_size: Option<u32>,
    pub lazy_validate_steps: Vec<LazyValidateStep>,
}

pub struct StructMemberOutput {
    pub name: String,
    pub type_ref: String,
    pub turbofish_type: String,
    pub serde_as_type: Option<String>,
    // Lazy fields
    pub lazy_type: String,
    pub lazy_is_scalar: bool,
    pub lazy_accessor: LazyAccessor,
}

pub struct EnumOutput {
    pub name: String,
    pub source_comment: String,
    pub has_default: bool,
    pub is_custom_str: bool,
    pub members: Vec<EnumStructMemberOutput>,
}

pub struct EnumStructMemberOutput {
    pub name: String,
    pub value: i32,
    pub is_first: bool,
}

pub struct UnionOutput {
    pub name: String,
    pub source_comment: String,
    pub has_default: bool,
    pub is_custom_str: bool,
    pub discriminant_type: String,
    pub arms: Vec<UnionArmOutput>,
    // Lazy fields
    pub lazy_name: String,
    pub lazy_discriminant_type: String,
    pub lazy_discriminant_is_enum: bool,
}

pub struct UnionArmOutput {
    pub case_name: String,
    pub case_value: String,
    pub is_void: bool,
    pub type_ref: Option<String>,
    pub turbofish_type: Option<String>,
    pub serde_as_type: Option<String>,
    // Lazy fields
    pub lazy_type: Option<String>,
    pub case_value_i32: String,
}

pub struct TypedefAliasOutput {
    pub name: String,
    pub source_comment: String,
    pub type_ref: String,
    // Lazy fields
    pub lazy_type: String,
}

pub struct TypedefNewtypeOutput {
    pub name: String,
    pub source_comment: String,
    pub has_default: bool,
    pub is_var_array: bool,
    pub is_fixed_opaque: bool,
    pub is_fixed_array: bool,
    pub is_custom_str: bool,
    pub type_ref: String,
    pub turbofish_type: String,
    pub serde_as_type: Option<String>,
    pub element_type: String,
    pub size: Option<String>,
    pub custom_debug: bool,
    pub custom_display_fromstr: bool,
    pub custom_schemars: bool,
    // Lazy fields
    pub lazy_name: String,
    pub lazy_inner_type: String,
    pub lazy_fixed_size: Option<u32>,
    pub lazy_inner_is_scalar: bool,
}

pub struct ConstOutput {
    pub name: String,
    pub doc_name: String,
    pub source_comment: String,
    pub value_str: String,
}

pub struct TypeEnumOutput {
    pub types: Vec<String>,
}

// =========================================================================
// Lazy support types (used as fields in the above)
// =========================================================================

pub enum LazyValidateStep {
    FixedGroup(LazyValidateFixedGroup),
    Variable(LazyValidateVariable),
}

pub struct LazyValidateFixedGroup {
    pub total_fixed: u32,
    pub content_validations: Vec<LazyContentValidation>,
}

pub struct LazyContentValidation {
    pub offset: u32,
    pub lazy_type: String,
}

pub struct LazyValidateVariable {
    pub lazy_type: String,
}

pub struct LazyAccessor {
    pub initial_fixed: u32,
    pub var_skips: Vec<LazyVarSkip>,
}

pub struct LazyVarSkip {
    pub lazy_type: String,
    pub post_fixed: u32,
}
