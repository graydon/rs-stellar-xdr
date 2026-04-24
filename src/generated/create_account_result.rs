#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// CreateAccountResult is an XDR Union defined as:
///
/// ```text
/// union CreateAccountResult switch (CreateAccountResultCode code)
/// {
/// case CREATE_ACCOUNT_SUCCESS:
///     void;
/// case CREATE_ACCOUNT_MALFORMED:
/// case CREATE_ACCOUNT_UNDERFUNDED:
/// case CREATE_ACCOUNT_LOW_RESERVE:
/// case CREATE_ACCOUNT_ALREADY_EXIST:
///     void;
/// };
/// ```
///
// union with discriminant CreateAccountResultCode
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
pub enum CreateAccountResult {
    Success,
    Malformed,
    Underfunded,
    LowReserve,
    AlreadyExist,
}

#[cfg(feature = "alloc")]
impl Default for CreateAccountResult {
    fn default() -> Self {
        Self::Success
    }
}

impl CreateAccountResult {
    const _VARIANTS: &[CreateAccountResultCode] = &[
        CreateAccountResultCode::Success,
        CreateAccountResultCode::Malformed,
        CreateAccountResultCode::Underfunded,
        CreateAccountResultCode::LowReserve,
        CreateAccountResultCode::AlreadyExist,
    ];
    pub const VARIANTS: [CreateAccountResultCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Success",
        "Malformed",
        "Underfunded",
        "LowReserve",
        "AlreadyExist",
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
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::Underfunded => "Underfunded",
            Self::LowReserve => "LowReserve",
            Self::AlreadyExist => "AlreadyExist",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> CreateAccountResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => CreateAccountResultCode::Success,
            Self::Malformed => CreateAccountResultCode::Malformed,
            Self::Underfunded => CreateAccountResultCode::Underfunded,
            Self::LowReserve => CreateAccountResultCode::LowReserve,
            Self::AlreadyExist => CreateAccountResultCode::AlreadyExist,
        }
    }

    #[must_use]
    pub const fn variants() -> [CreateAccountResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for CreateAccountResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<CreateAccountResultCode> for CreateAccountResult {
    #[must_use]
    fn discriminant(&self) -> CreateAccountResultCode {
        Self::discriminant(self)
    }
}

impl Variants<CreateAccountResultCode> for CreateAccountResult {
    fn variants() -> slice::Iter<'static, CreateAccountResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<CreateAccountResultCode> for CreateAccountResult {}

impl ReadXdr for CreateAccountResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: CreateAccountResultCode = <CreateAccountResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                CreateAccountResultCode::Success => Self::Success,
                CreateAccountResultCode::Malformed => Self::Malformed,
                CreateAccountResultCode::Underfunded => Self::Underfunded,
                CreateAccountResultCode::LowReserve => Self::LowReserve,
                CreateAccountResultCode::AlreadyExist => Self::AlreadyExist,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for CreateAccountResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::Underfunded => ().write_xdr(w)?,
                Self::LowReserve => ().write_xdr(w)?,
                Self::AlreadyExist => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`CreateAccountResult`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyCreateAccountResult(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyCreateAccountResult {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyCreateAccountResult {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyCreateAccountResult {
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
                // void — no additional data
            }
            -1 => {
                // void — no additional data
            }
            -2 => {
                // void — no additional data
            }
            -3 => {
                // void — no additional data
            }
            -4 => {
                // void — no additional data
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
                // void
            }
            -1 => {
                // void
            }
            -2 => {
                // void
            }
            -3 => {
                // void
            }
            -4 => {
                // void
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
impl From<LazyHandle> for LazyCreateAccountResult {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyCreateAccountResult {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyCreateAccountResult {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyCreateAccountResult {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> CreateAccountResultCode {
        // Validated — unwrap is safe.
        CreateAccountResultCode::try_from(self.discriminant_i32()).unwrap()
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&CreateAccountResult> for LazyCreateAccountResult {
    type Error = Error;
    fn try_from(val: &CreateAccountResult) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyCreateAccountResult> for CreateAccountResult {
    type Error = Error;
    fn try_from(lazy: &LazyCreateAccountResult) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
