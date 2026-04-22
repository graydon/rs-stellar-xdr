#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ScError is an XDR Union defined as:
///
/// ```text
/// union SCError switch (SCErrorType type)
/// {
/// case SCE_CONTRACT:
///     uint32 contractCode;
/// case SCE_WASM_VM:
/// case SCE_CONTEXT:
/// case SCE_STORAGE:
/// case SCE_OBJECT:
/// case SCE_CRYPTO:
/// case SCE_EVENTS:
/// case SCE_BUDGET:
/// case SCE_VALUE:
/// case SCE_AUTH:
///     SCErrorCode code;
/// };
/// ```
///
// union with discriminant ScErrorType
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
pub enum ScError {
    Contract(u32),
    WasmVm(ScErrorCode),
    Context(ScErrorCode),
    Storage(ScErrorCode),
    Object(ScErrorCode),
    Crypto(ScErrorCode),
    Events(ScErrorCode),
    Budget(ScErrorCode),
    Value(ScErrorCode),
    Auth(ScErrorCode),
}

#[cfg(feature = "alloc")]
impl Default for ScError {
    fn default() -> Self {
        Self::Contract(u32::default())
    }
}

impl ScError {
    const _VARIANTS: &[ScErrorType] = &[
        ScErrorType::Contract,
        ScErrorType::WasmVm,
        ScErrorType::Context,
        ScErrorType::Storage,
        ScErrorType::Object,
        ScErrorType::Crypto,
        ScErrorType::Events,
        ScErrorType::Budget,
        ScErrorType::Value,
        ScErrorType::Auth,
    ];
    pub const VARIANTS: [ScErrorType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Contract", "WasmVm", "Context", "Storage", "Object", "Crypto", "Events", "Budget",
        "Value", "Auth",
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
            Self::Contract(_) => "Contract",
            Self::WasmVm(_) => "WasmVm",
            Self::Context(_) => "Context",
            Self::Storage(_) => "Storage",
            Self::Object(_) => "Object",
            Self::Crypto(_) => "Crypto",
            Self::Events(_) => "Events",
            Self::Budget(_) => "Budget",
            Self::Value(_) => "Value",
            Self::Auth(_) => "Auth",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ScErrorType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Contract(_) => ScErrorType::Contract,
            Self::WasmVm(_) => ScErrorType::WasmVm,
            Self::Context(_) => ScErrorType::Context,
            Self::Storage(_) => ScErrorType::Storage,
            Self::Object(_) => ScErrorType::Object,
            Self::Crypto(_) => ScErrorType::Crypto,
            Self::Events(_) => ScErrorType::Events,
            Self::Budget(_) => ScErrorType::Budget,
            Self::Value(_) => ScErrorType::Value,
            Self::Auth(_) => ScErrorType::Auth,
        }
    }

    #[must_use]
    pub const fn variants() -> [ScErrorType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScError {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ScErrorType> for ScError {
    #[must_use]
    fn discriminant(&self) -> ScErrorType {
        Self::discriminant(self)
    }
}

impl Variants<ScErrorType> for ScError {
    fn variants() -> slice::Iter<'static, ScErrorType> {
        Self::VARIANTS.iter()
    }
}

impl Union<ScErrorType> for ScError {}

impl ReadXdr for ScError {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ScErrorType = <ScErrorType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ScErrorType::Contract => Self::Contract(u32::read_xdr(r)?),
                ScErrorType::WasmVm => Self::WasmVm(ScErrorCode::read_xdr(r)?),
                ScErrorType::Context => Self::Context(ScErrorCode::read_xdr(r)?),
                ScErrorType::Storage => Self::Storage(ScErrorCode::read_xdr(r)?),
                ScErrorType::Object => Self::Object(ScErrorCode::read_xdr(r)?),
                ScErrorType::Crypto => Self::Crypto(ScErrorCode::read_xdr(r)?),
                ScErrorType::Events => Self::Events(ScErrorCode::read_xdr(r)?),
                ScErrorType::Budget => Self::Budget(ScErrorCode::read_xdr(r)?),
                ScErrorType::Value => Self::Value(ScErrorCode::read_xdr(r)?),
                ScErrorType::Auth => Self::Auth(ScErrorCode::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ScError {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Contract(v) => v.write_xdr(w)?,
                Self::WasmVm(v) => v.write_xdr(w)?,
                Self::Context(v) => v.write_xdr(w)?,
                Self::Storage(v) => v.write_xdr(w)?,
                Self::Object(v) => v.write_xdr(w)?,
                Self::Crypto(v) => v.write_xdr(w)?,
                Self::Events(v) => v.write_xdr(w)?,
                Self::Budget(v) => v.write_xdr(w)?,
                Self::Value(v) => v.write_xdr(w)?,
                Self::Auth(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ScError`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyScError(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyScError {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyScError {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_contract().cmp(&other.as_contract()),
            1 => self.as_wasm_vm().cmp(&other.as_wasm_vm()),
            2 => self.as_context().cmp(&other.as_context()),
            3 => self.as_storage().cmp(&other.as_storage()),
            4 => self.as_object().cmp(&other.as_object()),
            5 => self.as_crypto().cmp(&other.as_crypto()),
            6 => self.as_events().cmp(&other.as_events()),
            7 => self.as_budget().cmp(&other.as_budget()),
            8 => self.as_value().cmp(&other.as_value()),
            9 => self.as_auth().cmp(&other.as_auth()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyScError {
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
                let field_len = <u32 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            1 => {
                let field_len =
                    <ScErrorCode as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            2 => {
                let field_len =
                    <ScErrorCode as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            3 => {
                let field_len =
                    <ScErrorCode as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            4 => {
                let field_len =
                    <ScErrorCode as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            5 => {
                let field_len =
                    <ScErrorCode as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            6 => {
                let field_len =
                    <ScErrorCode as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            7 => {
                let field_len =
                    <ScErrorCode as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            8 => {
                let field_len =
                    <ScErrorCode as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            9 => {
                let field_len =
                    <ScErrorCode as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
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
                pos += <u32 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            1 => {
                pos += <ScErrorCode as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            2 => {
                pos += <ScErrorCode as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            3 => {
                pos += <ScErrorCode as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            4 => {
                pos += <ScErrorCode as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            5 => {
                pos += <ScErrorCode as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            6 => {
                pos += <ScErrorCode as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            7 => {
                pos += <ScErrorCode as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            8 => {
                pos += <ScErrorCode as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            9 => {
                pos += <ScErrorCode as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyScError {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyScError {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyScError {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyScError {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> ScErrorType {
        // Validated — unwrap is safe.
        ScErrorType::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `Contract`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_contract(&self) -> Option<u32> {
        if self.discriminant_i32() == 0 {
            Some(<u32 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `WasmVm`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_wasm_vm(&self) -> Option<ScErrorCode> {
        if self.discriminant_i32() == 1 {
            Some(<ScErrorCode as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Context`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_context(&self) -> Option<ScErrorCode> {
        if self.discriminant_i32() == 2 {
            Some(<ScErrorCode as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Storage`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_storage(&self) -> Option<ScErrorCode> {
        if self.discriminant_i32() == 3 {
            Some(<ScErrorCode as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Object`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_object(&self) -> Option<ScErrorCode> {
        if self.discriminant_i32() == 4 {
            Some(<ScErrorCode as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Crypto`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_crypto(&self) -> Option<ScErrorCode> {
        if self.discriminant_i32() == 5 {
            Some(<ScErrorCode as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Events`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_events(&self) -> Option<ScErrorCode> {
        if self.discriminant_i32() == 6 {
            Some(<ScErrorCode as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Budget`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_budget(&self) -> Option<ScErrorCode> {
        if self.discriminant_i32() == 7 {
            Some(<ScErrorCode as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Value`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_value(&self) -> Option<ScErrorCode> {
        if self.discriminant_i32() == 8 {
            Some(<ScErrorCode as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Auth`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_auth(&self) -> Option<ScErrorCode> {
        if self.discriminant_i32() == 9 {
            Some(<ScErrorCode as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ScError> for LazyScError {
    type Error = Error;
    fn try_from(val: &ScError) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyScError> for ScError {
    type Error = Error;
    fn try_from(lazy: &LazyScError) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
