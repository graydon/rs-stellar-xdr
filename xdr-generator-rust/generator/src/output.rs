use askama::Template;

#[derive(Template)]
#[template(path = "generated.rs.jinja", escape = "none")]
pub struct GeneratedTemplate {
    pub xdr_files_sha256: Vec<(String, String)>,
    pub header: String,
    pub definitions: Vec<DefinitionOutput>,
    pub type_variant_enum: TypeEnumOutput,
}

#[derive(Template)]
#[template(path = "cxx_bridge.rs.jinja", escape = "none")]
pub struct CxxBridgeTemplate {
    pub bridge_types: Vec<CxxBridgeTypeOutput>,
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
    /// Compressed scan plan for computing the trusted length of the full struct.
    pub lazy_len_steps: Vec<LazyScanStepOutput>,
    pub member_names: String,
}

pub struct StructMemberOutput {
    pub name: String,
    pub type_ref: String,
    pub turbofish_type: String,
    pub serde_as_type: Option<String>,
    /// Resolved lazy metadata for the field type itself.
    pub lazy_type: LazyTypeOutput,
    /// Compressed scan plan for prior fields used to advance to this field.
    pub lazy_scan_steps: Vec<LazyScanStepOutput>,
}

pub struct EnumOutput {
    pub name: String,
    pub source_comment: String,
    pub has_default: bool,
    pub is_custom_str: bool,
    pub members: Vec<EnumStructMemberOutput>,
    /// Lazy metadata for reading and validating this enum's discriminant value.
    pub lazy_value: LazyValueOutput,
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
    /// Lazy metadata for reading and validating the union discriminant.
    pub lazy_discriminant: LazyValueOutput,
}

pub struct UnionArmOutput {
    pub case_name: String,
    pub case_value: String,
    pub is_void: bool,
    pub type_ref: Option<String>,
    pub turbofish_type: Option<String>,
    pub serde_as_type: Option<String>,
    pub lazy_method_name: String,
    /// Structured lazy form of the union case value used in matches and comparisons.
    pub lazy_case: LazyCaseOutput,
    /// Lazy metadata for the arm payload, when the arm is non-void.
    pub lazy_type: Option<LazyTypeOutput>,
}

pub struct TypedefAliasOutput {
    pub name: String,
    pub source_comment: String,
    pub type_ref: String,
    /// Lazy metadata for the aliased type as a handle-producing lazy wrapper.
    pub lazy_type: LazyTypeOutput,
    /// Lazy metadata for eagerly reading the builtin aliased value.
    pub lazy_value: LazyValueOutput,
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
    /// Lazy metadata for the wrapped inner type.
    pub lazy_inner_type: LazyTypeOutput,
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

pub struct CxxBridgeTypeOutput {
    pub name: String,
}

#[derive(Clone, Debug)]
pub enum LazyScanStepOutput {
    Fixed { len_expr: String },
    Variable { type_output: LazyTypeOutput },
}

impl LazyScanStepOutput {
    pub fn fixed(len_expr: String) -> Self {
        Self::Fixed { len_expr }
    }

    pub fn variable(type_output: LazyTypeOutput) -> Self {
        Self::Variable { type_output }
    }
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
    VarOpaque { max_size: String },
    String { max_size: String },
    Named { name: String },
    Optional { inner: Box<LazyTypeOutput> },
    Array { element: Box<LazyTypeOutput>, size: String },
    VarArray { element: Box<LazyTypeOutput>, max_size: String },
}

impl LazyTypeOutput {
    pub fn accessor_type(&self) -> String {
        match self {
            Self::I32 => "i32".to_string(),
            Self::U32 => "u32".to_string(),
            Self::I64 => "i64".to_string(),
            Self::U64 => "u64".to_string(),
            Self::Bool => "bool".to_string(),
            Self::F32 => "f32".to_string(),
            Self::F64 => "f64".to_string(),
            Self::FixedOpaque { size } => format!("[u8; {size}]"),
            _ => self.handle_type(),
        }
    }

    pub fn handle_type(&self) -> String {
        match self {
            Self::I32 => "i32".to_string(),
            Self::U32 => "u32".to_string(),
            Self::I64 => "i64".to_string(),
            Self::U64 => "u64".to_string(),
            Self::Bool => "bool".to_string(),
            Self::F32 => "f32".to_string(),
            Self::F64 => "f64".to_string(),
            Self::FixedOpaque { size } => format!("[u8; {size}]"),
            Self::VarOpaque { max_size } => format!("LazyBytesM<{}>", Self::const_arg(max_size)),
            Self::String { max_size } => format!("LazyStringM<{}>", Self::const_arg(max_size)),
            Self::Named { name } => name.clone(),
            Self::Optional { inner } => format!("LazyOption<{}>", inner.type_param()),
            Self::Array { element, size } => {
                format!("LazyArray<{}, {}>", element.type_param(), Self::const_arg(size))
            }
            Self::VarArray { element, max_size } => {
                format!("LazyVecM<{}, {}>", element.type_param(), Self::const_arg(max_size))
            }
        }
    }

    pub fn validate_expr(&self, buf: &str, offset: &str, limits: &str) -> String {
        match self {
            Self::I32 => format!("<i32 as LazyXdr>::lazy_validate({buf}, {offset}, {limits})?"),
            Self::U32 => format!("<u32 as LazyXdr>::lazy_validate({buf}, {offset}, {limits})?"),
            Self::I64 => format!("<i64 as LazyXdr>::lazy_validate({buf}, {offset}, {limits})?"),
            Self::U64 => format!("<u64 as LazyXdr>::lazy_validate({buf}, {offset}, {limits})?"),
            Self::Bool => format!("<bool as LazyXdr>::lazy_validate({buf}, {offset}, {limits})?"),
            Self::F32 | Self::F64 => "return Err(Error::Unsupported)".to_string(),
            _ => format!("<{} as LazyXdr>::lazy_validate({buf}, {offset}, {limits})?", self.handle_type()),
        }
    }

    pub fn validate_result_expr(&self, buf: &str, offset: &str, limits: &str) -> String {
        format!("Ok({})", self.validate_expr(buf, offset, limits))
    }

    pub fn trusted_len_expr(&self, buf: &str, offset: &str) -> String {
        match self {
            Self::I32 | Self::U32 | Self::Bool => "4".to_string(),
            Self::I64 | Self::U64 => "8".to_string(),
            Self::F32 => "4".to_string(),
            Self::F64 => "8".to_string(),
            Self::FixedOpaque { size } => {
                format!("{{ let len = u32::try_from({size}).map_err(|_| Error::LengthExceedsMax)?; len.checked_add(pad_len_ref(len)).ok_or(Error::LengthExceedsMax)? }}")
            }
            _ => format!("<{} as LazyXdr>::lazy_len({buf}, {offset})?", self.handle_type()),
        }
    }

    pub fn access_needs_ref(&self) -> bool {
        !matches!(
            self,
            Self::I32
                | Self::U32
                | Self::I64
                | Self::U64
                | Self::Bool
                | Self::F32
                | Self::F64
                | Self::FixedOpaque { .. }
        )
    }

    pub fn access_result_expr(&self, buf: &str, offset: &str, ref_name: &str) -> String {
        match self {
            Self::I32 => format!("read_i32_at({buf}, {offset})?"),
            Self::U32 => format!("read_u32_at({buf}, {offset})?"),
            Self::I64 => format!("read_i64_at({buf}, {offset})?"),
            Self::U64 => format!("read_u64_at({buf}, {offset})?"),
            Self::Bool => format!("read_bool_at({buf}, {offset})?"),
            Self::F32 => "return Err(Error::Unsupported)".to_string(),
            Self::F64 => "return Err(Error::Unsupported)".to_string(),
            Self::FixedOpaque { size } => format!(
                "read_bytes_at({buf}, {offset}, u32::try_from({size}).map_err(|_| Error::LengthExceedsMax)?)?.try_into().map_err(|_| Error::Invalid)?"
            ),
            _ => format!("<{} as TryFrom<LazyHandle>>::try_from({ref_name})?", self.handle_type()),
        }
    }

    fn type_param(&self) -> String {
        self.handle_type()
    }

    fn const_arg(value: &str) -> String {
        if value.chars().all(|character| character.is_ascii_digit()) {
            value.to_string()
        } else {
            format!("{{ {value} }}")
        }
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
    pub fn eager_type(&self) -> String {
        match self {
            Self::I32 => "i32".to_string(),
            Self::U32 => "u32".to_string(),
            Self::I64 => "i64".to_string(),
            Self::U64 => "u64".to_string(),
            Self::Bool => "bool".to_string(),
            Self::NamedEnum { name } => format!("super::{name}"),
        }
    }

    pub fn read_value_expr(&self, buf: &str, offset: &str) -> String {
        match self {
            Self::I32 => format!("read_i32_at({buf}, {offset})?"),
            Self::U32 => format!("read_u32_at({buf}, {offset})?"),
            Self::I64 => format!("read_i64_at({buf}, {offset})?"),
            Self::U64 => format!("read_u64_at({buf}, {offset})?"),
            Self::Bool => format!("read_bool_at({buf}, {offset})?"),
            Self::NamedEnum { name } => format!("{{ let value = read_i32_at({buf}, {offset})?; let value: super::{name} = value.try_into().map_err(|_| Error::Invalid)?; value }}"),
        }
    }

    pub fn read_result_expr(&self, buf: &str, offset: &str) -> String {
        format!("Ok({})", self.read_value_expr(buf, offset))
    }

    pub fn validate_expr(&self, buf: &str, offset: &str) -> String {
        match self {
            Self::I32 => format!("check_bounds({buf}, {offset}, 4)?; let _value = read_i32_at({buf}, {offset})?; Ok(4)"),
            Self::U32 => format!("check_bounds({buf}, {offset}, 4)?; let _value = read_u32_at({buf}, {offset})?; Ok(4)"),
            Self::I64 => format!("check_bounds({buf}, {offset}, 8)?; let _value = read_i64_at({buf}, {offset})?; Ok(8)"),
            Self::U64 => format!("check_bounds({buf}, {offset}, 8)?; let _value = read_u64_at({buf}, {offset})?; Ok(8)"),
            Self::Bool => format!("check_bounds({buf}, {offset}, 4)?; let _value = read_bool_at({buf}, {offset})?; Ok(4)"),
            Self::NamedEnum { name } => format!("check_bounds({buf}, {offset}, 4)?; let value = read_i32_at({buf}, {offset})?; let _: super::{name} = value.try_into().map_err(|_| Error::Invalid)?; Ok(4)"),
        }
    }

    pub fn trusted_len_expr(&self) -> &'static str {
        match self {
            Self::I32 | Self::U32 | Self::Bool | Self::NamedEnum { .. } => "4",
            Self::I64 | Self::U64 => "8",
        }
    }
}

#[derive(Clone, Debug)]
pub enum LazyCaseOutput {
    BareIdent { name: String },
    Literal { value: i32 },
    QualifiedVariant { type_name: String, variant_name: String },
    QualifiedLiteral { type_name: String, value: i32 },
}

impl LazyCaseOutput {
    pub fn render_expr(&self) -> String {
        match self {
            Self::BareIdent { name } => name.clone(),
            Self::Literal { value } => value.to_string(),
            Self::QualifiedVariant {
                type_name,
                variant_name,
            } => format!("{type_name}::{variant_name}"),
            Self::QualifiedLiteral { type_name, value } => format!("{type_name}({value})"),
        }
    }
}
