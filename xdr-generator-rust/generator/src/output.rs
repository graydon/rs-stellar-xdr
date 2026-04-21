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
    pub lazy_type: LazyTypeOutput,
}

pub struct CxxBridgeUnion {
    pub lazy_name: String,
    pub arms: Vec<CxxBridgeUnionArm>,
}

pub struct CxxBridgeUnionArm {
    pub is_void: bool,
    pub lazy_type: Option<LazyTypeOutput>,
    pub lazy_method_name: String,
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
    pub lazy_len_steps: Vec<LazyScanStepOutput>,
}

pub struct StructMemberOutput {
    pub name: String,
    pub type_ref: String,
    pub turbofish_type: String,
    pub serde_as_type: Option<String>,
    // Lazy fields
    pub lazy_type: LazyTypeOutput,
    pub lazy_scan_steps: Vec<LazyScanStepOutput>,
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
    pub lazy_discriminant: LazyValueOutput,
}

pub struct UnionArmOutput {
    pub case_name: String,
    pub case_value: String,
    pub case_value_i32: String,
    pub is_void: bool,
    pub type_ref: Option<String>,
    pub turbofish_type: Option<String>,
    pub serde_as_type: Option<String>,
    // Lazy fields
    pub lazy_type: Option<LazyTypeOutput>,
    pub lazy_method_name: String,
}

pub struct TypedefAliasOutput {
    pub name: String,
    pub source_comment: String,
    pub type_ref: String,
    // Lazy fields
    pub lazy_type: LazyTypeOutput,
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
    pub lazy_inner_type: LazyTypeOutput,
    pub lazy_fixed_size: Option<u32>,
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

#[derive(Clone, Debug)]
pub enum LazyScanStepOutput {
    Fixed { len_expr: String },
    Variable { type_output: LazyTypeOutput },
}

#[derive(Clone, Debug)]
pub enum LazyTypeOutput {
    I32,
    U32,
    I64,
    U64,
    Bool,
    F32,
    F64,
    FixedOpaque { size: String },
    VarOpaque { max_size: Option<String> },
    String { max_size: Option<String> },
    NamedEnum { name: String },
    NamedLazy { name: String },
    Optional { inner: Box<LazyTypeOutput> },
    Array { element: Box<LazyTypeOutput>, size: String },
    VarArray {
        element: Box<LazyTypeOutput>,
        max_size: Option<String>,
    },
}

impl LazyTypeOutput {
    pub fn rust_type(&self) -> String {
        match self {
            Self::I32 => "i32".to_string(),
            Self::U32 => "u32".to_string(),
            Self::I64 => "i64".to_string(),
            Self::U64 => "u64".to_string(),
            Self::Bool => "bool".to_string(),
            Self::F32 => "f32".to_string(),
            Self::F64 => "f64".to_string(),
            Self::FixedOpaque { size } => format!("LazyOpaqueFixed::<{size}>"),
            Self::VarOpaque { max_size } => match max_size {
                Some(size) => format!("LazyBytesM::<{size}>"),
                None => "LazyBytesM".to_string(),
            },
            Self::String { max_size } => match max_size {
                Some(size) => format!("LazyStringM::<{size}>"),
                None => "LazyStringM".to_string(),
            },
            Self::NamedEnum { name } => format!("super::{name}"),
            Self::NamedLazy { name } => format!("Lazy{name}"),
            Self::Optional { inner } => format!("LazyOption::<{}>", inner.rust_type()),
            Self::Array { element, size } => {
                format!("LazyFixedArray::<{}, {size}>", element.rust_type())
            }
            Self::VarArray { element, max_size } => match max_size {
                Some(size) => format!("LazyVecM::<{}, {size}>", element.rust_type()),
                None => format!("LazyVecM::<{}>", element.rust_type()),
            },
        }
    }

    pub fn is_scalar(&self) -> bool {
        matches!(
            self,
            Self::I32
                | Self::U32
                | Self::I64
                | Self::U64
                | Self::Bool
                | Self::F32
                | Self::F64
                | Self::NamedEnum { .. }
        )
    }

    pub fn cxx_bridge_type(&self) -> String {
        match self {
            Self::I32 => "i32".to_string(),
            Self::U32 => "u32".to_string(),
            Self::I64 => "i64".to_string(),
            Self::U64 => "u64".to_string(),
            Self::Bool => "bool".to_string(),
            Self::F32 => "f32".to_string(),
            Self::F64 => "f64".to_string(),
            Self::NamedEnum { .. } => "i32".to_string(),
            _ => self.rust_type(),
        }
    }

    pub fn cxx_scalar_cast_suffix(&self) -> &'static str {
        match self {
            Self::NamedEnum { .. } => " as i32",
            _ => "",
        }
    }

    pub fn validate_expr(&self, buf: &str, offset: &str) -> String {
        format!(
            "<{} as LazyXdr>::xdr_validate(&{buf}[{offset} as usize..])?",
            self.rust_type()
        )
    }

    pub fn len_expr(&self, buf: &str, offset: &str) -> String {
        format!(
            "<{} as LazyXdr>::xdr_len(&{buf}[{offset} as usize..])",
            self.rust_type()
        )
    }

    pub fn from_parent_expr(&self, parent: &str, offset: &str) -> String {
        format!(
            "<{} as LazyXdr>::from_xdr_at({parent}, {offset})",
            self.rust_type()
        )
    }
}

#[derive(Clone, Debug)]
pub enum LazyValueOutput {
    I32,
    U32,
    I64,
    U64,
    Bool,
    NamedEnum { name: String },
}

impl LazyValueOutput {
    pub fn rust_type(&self) -> String {
        match self {
            Self::I32 => "i32".to_string(),
            Self::U32 => "u32".to_string(),
            Self::I64 => "i64".to_string(),
            Self::U64 => "u64".to_string(),
            Self::Bool => "bool".to_string(),
            Self::NamedEnum { name } => format!("super::{name}"),
        }
    }

    pub fn from_parent_expr(&self, parent: &str, offset: &str) -> String {
        format!(
            "<{} as LazyXdr>::from_xdr_at({parent}, {offset})",
            self.rust_type()
        )
    }
}

