#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ScSpecTypeDef is an XDR Union defined as:
///
/// ```text
/// union SCSpecTypeDef switch (SCSpecType type)
/// {
/// case SC_SPEC_TYPE_VAL:
/// case SC_SPEC_TYPE_BOOL:
/// case SC_SPEC_TYPE_VOID:
/// case SC_SPEC_TYPE_ERROR:
/// case SC_SPEC_TYPE_U32:
/// case SC_SPEC_TYPE_I32:
/// case SC_SPEC_TYPE_U64:
/// case SC_SPEC_TYPE_I64:
/// case SC_SPEC_TYPE_TIMEPOINT:
/// case SC_SPEC_TYPE_DURATION:
/// case SC_SPEC_TYPE_U128:
/// case SC_SPEC_TYPE_I128:
/// case SC_SPEC_TYPE_U256:
/// case SC_SPEC_TYPE_I256:
/// case SC_SPEC_TYPE_BYTES:
/// case SC_SPEC_TYPE_STRING:
/// case SC_SPEC_TYPE_SYMBOL:
/// case SC_SPEC_TYPE_ADDRESS:
/// case SC_SPEC_TYPE_MUXED_ADDRESS:
///     void;
/// case SC_SPEC_TYPE_OPTION:
///     SCSpecTypeOption option;
/// case SC_SPEC_TYPE_RESULT:
///     SCSpecTypeResult result;
/// case SC_SPEC_TYPE_VEC:
///     SCSpecTypeVec vec;
/// case SC_SPEC_TYPE_MAP:
///     SCSpecTypeMap map;
/// case SC_SPEC_TYPE_TUPLE:
///     SCSpecTypeTuple tuple;
/// case SC_SPEC_TYPE_BYTES_N:
///     SCSpecTypeBytesN bytesN;
/// case SC_SPEC_TYPE_UDT:
///     SCSpecTypeUDT udt;
/// };
/// ```
///
// union with discriminant ScSpecType
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[allow(clippy::large_enum_variant)]
pub enum ScSpecTypeDef {
    Val,
    Bool,
    Void,
    Error,
    U32,
    I32,
    U64,
    I64,
    Timepoint,
    Duration,
    U128,
    I128,
    U256,
    I256,
    Bytes,
    String,
    Symbol,
    Address,
    MuxedAddress,
    Option(Box<ScSpecTypeOption>),
    Result(Box<ScSpecTypeResult>),
    Vec(Box<ScSpecTypeVec>),
    Map(Box<ScSpecTypeMap>),
    Tuple(Box<ScSpecTypeTuple>),
    BytesN(ScSpecTypeBytesN),
    Udt(ScSpecTypeUdt),
}

#[cfg(feature = "alloc")]
impl Default for ScSpecTypeDef {
    fn default() -> Self {
        Self::Val
    }
}

impl ScSpecTypeDef {
    const _VARIANTS: &[ScSpecType] = &[
        ScSpecType::Val,
        ScSpecType::Bool,
        ScSpecType::Void,
        ScSpecType::Error,
        ScSpecType::U32,
        ScSpecType::I32,
        ScSpecType::U64,
        ScSpecType::I64,
        ScSpecType::Timepoint,
        ScSpecType::Duration,
        ScSpecType::U128,
        ScSpecType::I128,
        ScSpecType::U256,
        ScSpecType::I256,
        ScSpecType::Bytes,
        ScSpecType::String,
        ScSpecType::Symbol,
        ScSpecType::Address,
        ScSpecType::MuxedAddress,
        ScSpecType::Option,
        ScSpecType::Result,
        ScSpecType::Vec,
        ScSpecType::Map,
        ScSpecType::Tuple,
        ScSpecType::BytesN,
        ScSpecType::Udt,
    ];
    pub const VARIANTS: [ScSpecType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Val",
        "Bool",
        "Void",
        "Error",
        "U32",
        "I32",
        "U64",
        "I64",
        "Timepoint",
        "Duration",
        "U128",
        "I128",
        "U256",
        "I256",
        "Bytes",
        "String",
        "Symbol",
        "Address",
        "MuxedAddress",
        "Option",
        "Result",
        "Vec",
        "Map",
        "Tuple",
        "BytesN",
        "Udt",
    ];
    pub const VARIANTS_STR: [&'static str; Self::_VARIANTS_STR.len()] = {
        let mut arr = [Self::_VARIANTS_STR[0]; Self::_VARIANTS_STR.len()];
        let mut i = 1;
        while i < Self::_VARIANTS_STR.len() {
            arr[i] = Self::_VARIANTS_STR[i];
            i += 1;
        }
        arr
    };

    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Val => "Val",
            Self::Bool => "Bool",
            Self::Void => "Void",
            Self::Error => "Error",
            Self::U32 => "U32",
            Self::I32 => "I32",
            Self::U64 => "U64",
            Self::I64 => "I64",
            Self::Timepoint => "Timepoint",
            Self::Duration => "Duration",
            Self::U128 => "U128",
            Self::I128 => "I128",
            Self::U256 => "U256",
            Self::I256 => "I256",
            Self::Bytes => "Bytes",
            Self::String => "String",
            Self::Symbol => "Symbol",
            Self::Address => "Address",
            Self::MuxedAddress => "MuxedAddress",
            Self::Option(_) => "Option",
            Self::Result(_) => "Result",
            Self::Vec(_) => "Vec",
            Self::Map(_) => "Map",
            Self::Tuple(_) => "Tuple",
            Self::BytesN(_) => "BytesN",
            Self::Udt(_) => "Udt",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ScSpecType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Val => ScSpecType::Val,
            Self::Bool => ScSpecType::Bool,
            Self::Void => ScSpecType::Void,
            Self::Error => ScSpecType::Error,
            Self::U32 => ScSpecType::U32,
            Self::I32 => ScSpecType::I32,
            Self::U64 => ScSpecType::U64,
            Self::I64 => ScSpecType::I64,
            Self::Timepoint => ScSpecType::Timepoint,
            Self::Duration => ScSpecType::Duration,
            Self::U128 => ScSpecType::U128,
            Self::I128 => ScSpecType::I128,
            Self::U256 => ScSpecType::U256,
            Self::I256 => ScSpecType::I256,
            Self::Bytes => ScSpecType::Bytes,
            Self::String => ScSpecType::String,
            Self::Symbol => ScSpecType::Symbol,
            Self::Address => ScSpecType::Address,
            Self::MuxedAddress => ScSpecType::MuxedAddress,
            Self::Option(_) => ScSpecType::Option,
            Self::Result(_) => ScSpecType::Result,
            Self::Vec(_) => ScSpecType::Vec,
            Self::Map(_) => ScSpecType::Map,
            Self::Tuple(_) => ScSpecType::Tuple,
            Self::BytesN(_) => ScSpecType::BytesN,
            Self::Udt(_) => ScSpecType::Udt,
        }
    }

    #[must_use]
    pub const fn variants() -> [ScSpecType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScSpecTypeDef {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ScSpecType> for ScSpecTypeDef {
    #[must_use]
    fn discriminant(&self) -> ScSpecType {
        Self::discriminant(self)
    }
}

impl Variants<ScSpecType> for ScSpecTypeDef {
    fn variants() -> slice::Iter<'static, ScSpecType> {
        Self::VARIANTS.iter()
    }
}

impl Union<ScSpecType> for ScSpecTypeDef {}

impl ReadXdr for ScSpecTypeDef {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ScSpecType = <ScSpecType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ScSpecType::Val => Self::Val,
                ScSpecType::Bool => Self::Bool,
                ScSpecType::Void => Self::Void,
                ScSpecType::Error => Self::Error,
                ScSpecType::U32 => Self::U32,
                ScSpecType::I32 => Self::I32,
                ScSpecType::U64 => Self::U64,
                ScSpecType::I64 => Self::I64,
                ScSpecType::Timepoint => Self::Timepoint,
                ScSpecType::Duration => Self::Duration,
                ScSpecType::U128 => Self::U128,
                ScSpecType::I128 => Self::I128,
                ScSpecType::U256 => Self::U256,
                ScSpecType::I256 => Self::I256,
                ScSpecType::Bytes => Self::Bytes,
                ScSpecType::String => Self::String,
                ScSpecType::Symbol => Self::Symbol,
                ScSpecType::Address => Self::Address,
                ScSpecType::MuxedAddress => Self::MuxedAddress,
                ScSpecType::Option => Self::Option(Box::<ScSpecTypeOption>::read_xdr(r)?),
                ScSpecType::Result => Self::Result(Box::<ScSpecTypeResult>::read_xdr(r)?),
                ScSpecType::Vec => Self::Vec(Box::<ScSpecTypeVec>::read_xdr(r)?),
                ScSpecType::Map => Self::Map(Box::<ScSpecTypeMap>::read_xdr(r)?),
                ScSpecType::Tuple => Self::Tuple(Box::<ScSpecTypeTuple>::read_xdr(r)?),
                ScSpecType::BytesN => Self::BytesN(ScSpecTypeBytesN::read_xdr(r)?),
                ScSpecType::Udt => Self::Udt(ScSpecTypeUdt::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ScSpecTypeDef {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Val => ().write_xdr(w)?,
                Self::Bool => ().write_xdr(w)?,
                Self::Void => ().write_xdr(w)?,
                Self::Error => ().write_xdr(w)?,
                Self::U32 => ().write_xdr(w)?,
                Self::I32 => ().write_xdr(w)?,
                Self::U64 => ().write_xdr(w)?,
                Self::I64 => ().write_xdr(w)?,
                Self::Timepoint => ().write_xdr(w)?,
                Self::Duration => ().write_xdr(w)?,
                Self::U128 => ().write_xdr(w)?,
                Self::I128 => ().write_xdr(w)?,
                Self::U256 => ().write_xdr(w)?,
                Self::I256 => ().write_xdr(w)?,
                Self::Bytes => ().write_xdr(w)?,
                Self::String => ().write_xdr(w)?,
                Self::Symbol => ().write_xdr(w)?,
                Self::Address => ().write_xdr(w)?,
                Self::MuxedAddress => ().write_xdr(w)?,
                Self::Option(v) => v.write_xdr(w)?,
                Self::Result(v) => v.write_xdr(w)?,
                Self::Vec(v) => v.write_xdr(w)?,
                Self::Map(v) => v.write_xdr(w)?,
                Self::Tuple(v) => v.write_xdr(w)?,
                Self::BytesN(v) => v.write_xdr(w)?,
                Self::Udt(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ScSpecTypeDef`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyScSpecTypeDef(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyScSpecTypeDef {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyScSpecTypeDef {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            1000 => self.as_option().cmp(&other.as_option()),
            1001 => self.as_result().cmp(&other.as_result()),
            1002 => self.as_vec().cmp(&other.as_vec()),
            1004 => self.as_map().cmp(&other.as_map()),
            1005 => self.as_tuple().cmp(&other.as_tuple()),
            1006 => self.as_bytes_n().cmp(&other.as_bytes_n()),
            2000 => self.as_udt().cmp(&other.as_udt()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyScSpecTypeDef {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let disc = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        #[allow(unused_mut)]
        let mut pos: u32 = 4;
        #[allow(clippy::match_same_arms)]
        match disc {
            0 => {
                // void — no additional data
            }
            1 => {
                // void — no additional data
            }
            2 => {
                // void — no additional data
            }
            3 => {
                // void — no additional data
            }
            4 => {
                // void — no additional data
            }
            5 => {
                // void — no additional data
            }
            6 => {
                // void — no additional data
            }
            7 => {
                // void — no additional data
            }
            8 => {
                // void — no additional data
            }
            9 => {
                // void — no additional data
            }
            10 => {
                // void — no additional data
            }
            11 => {
                // void — no additional data
            }
            12 => {
                // void — no additional data
            }
            13 => {
                // void — no additional data
            }
            14 => {
                // void — no additional data
            }
            16 => {
                // void — no additional data
            }
            17 => {
                // void — no additional data
            }
            19 => {
                // void — no additional data
            }
            20 => {
                // void — no additional data
            }
            1000 => {
                let field_len =
                    <LazyScSpecTypeOption as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            1001 => {
                let field_len =
                    <LazyScSpecTypeResult as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            1002 => {
                let field_len =
                    <LazyScSpecTypeVec as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            1004 => {
                let field_len =
                    <LazyScSpecTypeMap as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            1005 => {
                let field_len =
                    <LazyScSpecTypeTuple as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            1006 => {
                let field_len =
                    <LazyScSpecTypeBytesN as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            2000 => {
                let field_len =
                    <LazyScSpecTypeUdt as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            _ => return Err(Error::Invalid),
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let disc = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        #[allow(unused_mut)]
        let mut pos: u32 = 4;
        #[allow(clippy::match_same_arms)]
        match disc {
            0 => {
                // void
            }
            1 => {
                // void
            }
            2 => {
                // void
            }
            3 => {
                // void
            }
            4 => {
                // void
            }
            5 => {
                // void
            }
            6 => {
                // void
            }
            7 => {
                // void
            }
            8 => {
                // void
            }
            9 => {
                // void
            }
            10 => {
                // void
            }
            11 => {
                // void
            }
            12 => {
                // void
            }
            13 => {
                // void
            }
            14 => {
                // void
            }
            16 => {
                // void
            }
            17 => {
                // void
            }
            19 => {
                // void
            }
            20 => {
                // void
            }
            1000 => {
                pos += <LazyScSpecTypeOption as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            1001 => {
                pos += <LazyScSpecTypeResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            1002 => {
                pos += <LazyScSpecTypeVec as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            1004 => {
                pos += <LazyScSpecTypeMap as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            1005 => {
                pos += <LazyScSpecTypeTuple as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            1006 => {
                pos += <LazyScSpecTypeBytesN as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            2000 => {
                pos += <LazyScSpecTypeUdt as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            _ => {}
        }
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyScSpecTypeDef {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyScSpecTypeDef {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyScSpecTypeDef {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyScSpecTypeDef {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> ScSpecType {
        // Validated — unwrap is safe.
        ScSpecType::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `Option`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_option(&self) -> Option<LazyScSpecTypeOption> {
        if self.discriminant_i32() == 1000 {
            Some(<LazyScSpecTypeOption as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Result`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_result(&self) -> Option<LazyScSpecTypeResult> {
        if self.discriminant_i32() == 1001 {
            Some(<LazyScSpecTypeResult as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Vec`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_vec(&self) -> Option<LazyScSpecTypeVec> {
        if self.discriminant_i32() == 1002 {
            Some(<LazyScSpecTypeVec as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Map`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_map(&self) -> Option<LazyScSpecTypeMap> {
        if self.discriminant_i32() == 1004 {
            Some(<LazyScSpecTypeMap as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Tuple`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_tuple(&self) -> Option<LazyScSpecTypeTuple> {
        if self.discriminant_i32() == 1005 {
            Some(<LazyScSpecTypeTuple as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `BytesN`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_bytes_n(&self) -> Option<LazyScSpecTypeBytesN> {
        if self.discriminant_i32() == 1006 {
            Some(<LazyScSpecTypeBytesN as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Udt`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_udt(&self) -> Option<LazyScSpecTypeUdt> {
        if self.discriminant_i32() == 2000 {
            Some(<LazyScSpecTypeUdt as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ScSpecTypeDef> for LazyScSpecTypeDef {
    type Error = Error;
    fn try_from(val: &ScSpecTypeDef) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyScSpecTypeDef> for ScSpecTypeDef {
    type Error = Error;
    fn try_from(lazy: &LazyScSpecTypeDef) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
