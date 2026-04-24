#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ScVal is an XDR Union defined as:
///
/// ```text
/// union SCVal switch (SCValType type)
/// {
///
/// case SCV_BOOL:
///     bool b;
/// case SCV_VOID:
///     void;
/// case SCV_ERROR:
///     SCError error;
///
/// case SCV_U32:
///     uint32 u32;
/// case SCV_I32:
///     int32 i32;
///
/// case SCV_U64:
///     uint64 u64;
/// case SCV_I64:
///     int64 i64;
/// case SCV_TIMEPOINT:
///     TimePoint timepoint;
/// case SCV_DURATION:
///     Duration duration;
///
/// case SCV_U128:
///     UInt128Parts u128;
/// case SCV_I128:
///     Int128Parts i128;
///
/// case SCV_U256:
///     UInt256Parts u256;
/// case SCV_I256:
///     Int256Parts i256;
///
/// case SCV_BYTES:
///     SCBytes bytes;
/// case SCV_STRING:
///     SCString str;
/// case SCV_SYMBOL:
///     SCSymbol sym;
///
/// // Vec and Map are recursive so need to live
/// // behind an option, due to xdrpp limitations.
/// case SCV_VEC:
///     SCVec *vec;
/// case SCV_MAP:
///     SCMap *map;
///
/// case SCV_ADDRESS:
///     SCAddress address;
///
/// // Special SCVals reserved for system-constructed contract-data
/// // ledger keys, not generally usable elsewhere.
/// case SCV_CONTRACT_INSTANCE:
///     SCContractInstance instance;
/// case SCV_LEDGER_KEY_CONTRACT_INSTANCE:
///     void;
/// case SCV_LEDGER_KEY_NONCE:
///     SCNonceKey nonce_key;
/// };
/// ```
///
// union with discriminant ScValType
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
pub enum ScVal {
    Bool(bool),
    Void,
    Error(ScError),
    U32(u32),
    I32(i32),
    U64(
        #[cfg_attr(
            all(feature = "serde", feature = "alloc"),
            serde_as(as = "NumberOrString")
        )]
        u64,
    ),
    I64(
        #[cfg_attr(
            all(feature = "serde", feature = "alloc"),
            serde_as(as = "NumberOrString")
        )]
        i64,
    ),
    Timepoint(TimePoint),
    Duration(Duration),
    U128(UInt128Parts),
    I128(Int128Parts),
    U256(UInt256Parts),
    I256(Int256Parts),
    Bytes(ScBytes),
    String(ScString),
    Symbol(ScSymbol),
    Vec(Option<ScVec>),
    Map(Option<ScMap>),
    Address(ScAddress),
    ContractInstance(ScContractInstance),
    LedgerKeyContractInstance,
    LedgerKeyNonce(ScNonceKey),
}

#[cfg(feature = "alloc")]
impl Default for ScVal {
    fn default() -> Self {
        Self::Bool(bool::default())
    }
}

impl ScVal {
    const _VARIANTS: &[ScValType] = &[
        ScValType::Bool,
        ScValType::Void,
        ScValType::Error,
        ScValType::U32,
        ScValType::I32,
        ScValType::U64,
        ScValType::I64,
        ScValType::Timepoint,
        ScValType::Duration,
        ScValType::U128,
        ScValType::I128,
        ScValType::U256,
        ScValType::I256,
        ScValType::Bytes,
        ScValType::String,
        ScValType::Symbol,
        ScValType::Vec,
        ScValType::Map,
        ScValType::Address,
        ScValType::ContractInstance,
        ScValType::LedgerKeyContractInstance,
        ScValType::LedgerKeyNonce,
    ];
    pub const VARIANTS: [ScValType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
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
        "Vec",
        "Map",
        "Address",
        "ContractInstance",
        "LedgerKeyContractInstance",
        "LedgerKeyNonce",
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
            Self::Bool(_) => "Bool",
            Self::Void => "Void",
            Self::Error(_) => "Error",
            Self::U32(_) => "U32",
            Self::I32(_) => "I32",
            Self::U64(_) => "U64",
            Self::I64(_) => "I64",
            Self::Timepoint(_) => "Timepoint",
            Self::Duration(_) => "Duration",
            Self::U128(_) => "U128",
            Self::I128(_) => "I128",
            Self::U256(_) => "U256",
            Self::I256(_) => "I256",
            Self::Bytes(_) => "Bytes",
            Self::String(_) => "String",
            Self::Symbol(_) => "Symbol",
            Self::Vec(_) => "Vec",
            Self::Map(_) => "Map",
            Self::Address(_) => "Address",
            Self::ContractInstance(_) => "ContractInstance",
            Self::LedgerKeyContractInstance => "LedgerKeyContractInstance",
            Self::LedgerKeyNonce(_) => "LedgerKeyNonce",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ScValType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Bool(_) => ScValType::Bool,
            Self::Void => ScValType::Void,
            Self::Error(_) => ScValType::Error,
            Self::U32(_) => ScValType::U32,
            Self::I32(_) => ScValType::I32,
            Self::U64(_) => ScValType::U64,
            Self::I64(_) => ScValType::I64,
            Self::Timepoint(_) => ScValType::Timepoint,
            Self::Duration(_) => ScValType::Duration,
            Self::U128(_) => ScValType::U128,
            Self::I128(_) => ScValType::I128,
            Self::U256(_) => ScValType::U256,
            Self::I256(_) => ScValType::I256,
            Self::Bytes(_) => ScValType::Bytes,
            Self::String(_) => ScValType::String,
            Self::Symbol(_) => ScValType::Symbol,
            Self::Vec(_) => ScValType::Vec,
            Self::Map(_) => ScValType::Map,
            Self::Address(_) => ScValType::Address,
            Self::ContractInstance(_) => ScValType::ContractInstance,
            Self::LedgerKeyContractInstance => ScValType::LedgerKeyContractInstance,
            Self::LedgerKeyNonce(_) => ScValType::LedgerKeyNonce,
        }
    }

    #[must_use]
    pub const fn variants() -> [ScValType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScVal {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ScValType> for ScVal {
    #[must_use]
    fn discriminant(&self) -> ScValType {
        Self::discriminant(self)
    }
}

impl Variants<ScValType> for ScVal {
    fn variants() -> slice::Iter<'static, ScValType> {
        Self::VARIANTS.iter()
    }
}

impl Union<ScValType> for ScVal {}

impl ReadXdr for ScVal {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ScValType = <ScValType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ScValType::Bool => Self::Bool(bool::read_xdr(r)?),
                ScValType::Void => Self::Void,
                ScValType::Error => Self::Error(ScError::read_xdr(r)?),
                ScValType::U32 => Self::U32(u32::read_xdr(r)?),
                ScValType::I32 => Self::I32(i32::read_xdr(r)?),
                ScValType::U64 => Self::U64(u64::read_xdr(r)?),
                ScValType::I64 => Self::I64(i64::read_xdr(r)?),
                ScValType::Timepoint => Self::Timepoint(TimePoint::read_xdr(r)?),
                ScValType::Duration => Self::Duration(Duration::read_xdr(r)?),
                ScValType::U128 => Self::U128(UInt128Parts::read_xdr(r)?),
                ScValType::I128 => Self::I128(Int128Parts::read_xdr(r)?),
                ScValType::U256 => Self::U256(UInt256Parts::read_xdr(r)?),
                ScValType::I256 => Self::I256(Int256Parts::read_xdr(r)?),
                ScValType::Bytes => Self::Bytes(ScBytes::read_xdr(r)?),
                ScValType::String => Self::String(ScString::read_xdr(r)?),
                ScValType::Symbol => Self::Symbol(ScSymbol::read_xdr(r)?),
                ScValType::Vec => Self::Vec(Option::<ScVec>::read_xdr(r)?),
                ScValType::Map => Self::Map(Option::<ScMap>::read_xdr(r)?),
                ScValType::Address => Self::Address(ScAddress::read_xdr(r)?),
                ScValType::ContractInstance => {
                    Self::ContractInstance(ScContractInstance::read_xdr(r)?)
                }
                ScValType::LedgerKeyContractInstance => Self::LedgerKeyContractInstance,
                ScValType::LedgerKeyNonce => Self::LedgerKeyNonce(ScNonceKey::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ScVal {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Bool(v) => v.write_xdr(w)?,
                Self::Void => ().write_xdr(w)?,
                Self::Error(v) => v.write_xdr(w)?,
                Self::U32(v) => v.write_xdr(w)?,
                Self::I32(v) => v.write_xdr(w)?,
                Self::U64(v) => v.write_xdr(w)?,
                Self::I64(v) => v.write_xdr(w)?,
                Self::Timepoint(v) => v.write_xdr(w)?,
                Self::Duration(v) => v.write_xdr(w)?,
                Self::U128(v) => v.write_xdr(w)?,
                Self::I128(v) => v.write_xdr(w)?,
                Self::U256(v) => v.write_xdr(w)?,
                Self::I256(v) => v.write_xdr(w)?,
                Self::Bytes(v) => v.write_xdr(w)?,
                Self::String(v) => v.write_xdr(w)?,
                Self::Symbol(v) => v.write_xdr(w)?,
                Self::Vec(v) => v.write_xdr(w)?,
                Self::Map(v) => v.write_xdr(w)?,
                Self::Address(v) => v.write_xdr(w)?,
                Self::ContractInstance(v) => v.write_xdr(w)?,
                Self::LedgerKeyContractInstance => ().write_xdr(w)?,
                Self::LedgerKeyNonce(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ScVal`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyScVal(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyScVal {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyScVal {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_bool().cmp(&other.as_bool()),
            2 => self.as_error().cmp(&other.as_error()),
            3 => self.as_u32().cmp(&other.as_u32()),
            4 => self.as_i32().cmp(&other.as_i32()),
            5 => self.as_u64().cmp(&other.as_u64()),
            6 => self.as_i64().cmp(&other.as_i64()),
            7 => self.as_timepoint().cmp(&other.as_timepoint()),
            8 => self.as_duration().cmp(&other.as_duration()),
            9 => self.as_u128().cmp(&other.as_u128()),
            10 => self.as_i128().cmp(&other.as_i128()),
            11 => self.as_u256().cmp(&other.as_u256()),
            12 => self.as_i256().cmp(&other.as_i256()),
            13 => self.as_bytes().cmp(&other.as_bytes()),
            14 => self.as_string().cmp(&other.as_string()),
            15 => self.as_symbol().cmp(&other.as_symbol()),
            16 => self.as_vec().cmp(&other.as_vec()),
            17 => self.as_map().cmp(&other.as_map()),
            18 => self.as_address().cmp(&other.as_address()),
            19 => self
                .as_contract_instance()
                .cmp(&other.as_contract_instance()),
            21 => self.as_ledger_key_nonce().cmp(&other.as_ledger_key_nonce()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyScVal {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let bytes: [u8; 4] = buf[..4].try_into().unwrap();
        let disc = i32::from_be_bytes(bytes);
        #[allow(unused_mut)]
        let mut pos: u32 = 4;
        #[allow(clippy::match_same_arms)]
        match disc {
            0 => {
                let field_len = <bool as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            1 => {
                // void — no additional data
            }
            2 => {
                let field_len =
                    <LazyScError as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            3 => {
                let field_len = <u32 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            4 => {
                let field_len = <i32 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            5 => {
                let field_len = <u64 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            6 => {
                let field_len = <i64 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            7 => {
                let field_len =
                    <LazyTimePoint as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            8 => {
                let field_len =
                    <LazyDuration as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            9 => {
                let field_len =
                    <LazyUInt128Parts as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            10 => {
                let field_len =
                    <LazyInt128Parts as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            11 => {
                let field_len =
                    <LazyUInt256Parts as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            12 => {
                let field_len =
                    <LazyInt256Parts as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            13 => {
                let field_len =
                    <LazyScBytes as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            14 => {
                let field_len =
                    <LazyScString as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            15 => {
                let field_len =
                    <LazyScSymbol as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            16 => {
                let field_len =
                    <LazyOption<LazyScVec> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            17 => {
                let field_len =
                    <LazyOption<LazyScMap> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            18 => {
                let field_len =
                    <LazyScAddress as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            19 => {
                let field_len =
                    <LazyScContractInstance as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            20 => {
                // void — no additional data
            }
            21 => {
                let field_len =
                    <LazyScNonceKey as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            _ => return Err(Error::Invalid),
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let bytes: [u8; 4] = buf[..4].try_into().unwrap();
        let disc = i32::from_be_bytes(bytes);
        #[allow(unused_mut)]
        let mut pos: u32 = 4;
        #[allow(clippy::match_same_arms)]
        match disc {
            0 => {
                pos += <bool as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            1 => {
                // void
            }
            2 => {
                pos += <LazyScError as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            3 => {
                pos += <u32 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            4 => {
                pos += <i32 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            5 => {
                pos += <u64 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            6 => {
                pos += <i64 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            7 => {
                pos += <LazyTimePoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            8 => {
                pos += <LazyDuration as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            9 => {
                pos += <LazyUInt128Parts as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            10 => {
                pos += <LazyInt128Parts as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            11 => {
                pos += <LazyUInt256Parts as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            12 => {
                pos += <LazyInt256Parts as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            13 => {
                pos += <LazyScBytes as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            14 => {
                pos += <LazyScString as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            15 => {
                pos += <LazyScSymbol as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            16 => {
                pos += <LazyOption<LazyScVec> as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            17 => {
                pos += <LazyOption<LazyScMap> as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            18 => {
                pos += <LazyScAddress as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            19 => {
                pos += <LazyScContractInstance as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            20 => {
                // void
            }
            21 => {
                pos += <LazyScNonceKey as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            _ => {}
        }
        pos
    }

    fn from_xdr_consume(parent: &LazyHandle, buf: &mut &[u8]) -> Self {
        let len = Self::xdr_len(buf);
        let offset = (parent.len() as usize - buf.len()) as u32;
        let handle = parent.sub_handle(offset, len);
        *buf = &buf[len as usize..];
        Self(handle)
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyScVal {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyScVal {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyScVal {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyScVal {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> ScValType {
        // Validated — unwrap is safe.
        ScValType::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `Bool`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_bool(&self) -> Option<bool> {
        if self.discriminant_i32() == 0 {
            Some(<bool as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Error`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_error(&self) -> Option<LazyScError> {
        if self.discriminant_i32() == 2 {
            Some(<LazyScError as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `U32`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_u32(&self) -> Option<u32> {
        if self.discriminant_i32() == 3 {
            Some(<u32 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `I32`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_i32(&self) -> Option<i32> {
        if self.discriminant_i32() == 4 {
            Some(<i32 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `U64`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_u64(&self) -> Option<u64> {
        if self.discriminant_i32() == 5 {
            Some(<u64 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `I64`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_i64(&self) -> Option<i64> {
        if self.discriminant_i32() == 6 {
            Some(<i64 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Timepoint`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_timepoint(&self) -> Option<LazyTimePoint> {
        if self.discriminant_i32() == 7 {
            Some(<LazyTimePoint as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Duration`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_duration(&self) -> Option<LazyDuration> {
        if self.discriminant_i32() == 8 {
            Some(<LazyDuration as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `U128`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_u128(&self) -> Option<LazyUInt128Parts> {
        if self.discriminant_i32() == 9 {
            Some(<LazyUInt128Parts as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `I128`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_i128(&self) -> Option<LazyInt128Parts> {
        if self.discriminant_i32() == 10 {
            Some(<LazyInt128Parts as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `U256`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_u256(&self) -> Option<LazyUInt256Parts> {
        if self.discriminant_i32() == 11 {
            Some(<LazyUInt256Parts as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `I256`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_i256(&self) -> Option<LazyInt256Parts> {
        if self.discriminant_i32() == 12 {
            Some(<LazyInt256Parts as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Bytes`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_bytes(&self) -> Option<LazyScBytes> {
        if self.discriminant_i32() == 13 {
            Some(<LazyScBytes as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `String`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_string(&self) -> Option<LazyScString> {
        if self.discriminant_i32() == 14 {
            Some(<LazyScString as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Symbol`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_symbol(&self) -> Option<LazyScSymbol> {
        if self.discriminant_i32() == 15 {
            Some(<LazyScSymbol as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Vec`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_vec(&self) -> Option<LazyOption<LazyScVec>> {
        if self.discriminant_i32() == 16 {
            Some(<LazyOption<LazyScVec> as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Map`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_map(&self) -> Option<LazyOption<LazyScMap>> {
        if self.discriminant_i32() == 17 {
            Some(<LazyOption<LazyScMap> as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Address`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_address(&self) -> Option<LazyScAddress> {
        if self.discriminant_i32() == 18 {
            Some(<LazyScAddress as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ContractInstance`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_contract_instance(&self) -> Option<LazyScContractInstance> {
        if self.discriminant_i32() == 19 {
            Some(<LazyScContractInstance as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `LedgerKeyNonce`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_ledger_key_nonce(&self) -> Option<LazyScNonceKey> {
        if self.discriminant_i32() == 21 {
            Some(<LazyScNonceKey as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ScVal> for LazyScVal {
    type Error = Error;
    fn try_from(val: &ScVal) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyScVal> for ScVal {
    type Error = Error;
    fn try_from(lazy: &LazyScVal) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
