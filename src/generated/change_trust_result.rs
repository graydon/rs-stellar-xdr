#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ChangeTrustResult is an XDR Union defined as:
///
/// ```text
/// union ChangeTrustResult switch (ChangeTrustResultCode code)
/// {
/// case CHANGE_TRUST_SUCCESS:
///     void;
/// case CHANGE_TRUST_MALFORMED:
/// case CHANGE_TRUST_NO_ISSUER:
/// case CHANGE_TRUST_INVALID_LIMIT:
/// case CHANGE_TRUST_LOW_RESERVE:
/// case CHANGE_TRUST_SELF_NOT_ALLOWED:
/// case CHANGE_TRUST_TRUST_LINE_MISSING:
/// case CHANGE_TRUST_CANNOT_DELETE:
/// case CHANGE_TRUST_NOT_AUTH_MAINTAIN_LIABILITIES:
///     void;
/// };
/// ```
///
// union with discriminant ChangeTrustResultCode
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
pub enum ChangeTrustResult {
    Success,
    Malformed,
    NoIssuer,
    InvalidLimit,
    LowReserve,
    SelfNotAllowed,
    TrustLineMissing,
    CannotDelete,
    NotAuthMaintainLiabilities,
}

#[cfg(feature = "alloc")]
impl Default for ChangeTrustResult {
    fn default() -> Self {
        Self::Success
    }
}

impl ChangeTrustResult {
    const _VARIANTS: &[ChangeTrustResultCode] = &[
        ChangeTrustResultCode::Success,
        ChangeTrustResultCode::Malformed,
        ChangeTrustResultCode::NoIssuer,
        ChangeTrustResultCode::InvalidLimit,
        ChangeTrustResultCode::LowReserve,
        ChangeTrustResultCode::SelfNotAllowed,
        ChangeTrustResultCode::TrustLineMissing,
        ChangeTrustResultCode::CannotDelete,
        ChangeTrustResultCode::NotAuthMaintainLiabilities,
    ];
    pub const VARIANTS: [ChangeTrustResultCode; Self::_VARIANTS.len()] = {
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
        "NoIssuer",
        "InvalidLimit",
        "LowReserve",
        "SelfNotAllowed",
        "TrustLineMissing",
        "CannotDelete",
        "NotAuthMaintainLiabilities",
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
            Self::NoIssuer => "NoIssuer",
            Self::InvalidLimit => "InvalidLimit",
            Self::LowReserve => "LowReserve",
            Self::SelfNotAllowed => "SelfNotAllowed",
            Self::TrustLineMissing => "TrustLineMissing",
            Self::CannotDelete => "CannotDelete",
            Self::NotAuthMaintainLiabilities => "NotAuthMaintainLiabilities",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ChangeTrustResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ChangeTrustResultCode::Success,
            Self::Malformed => ChangeTrustResultCode::Malformed,
            Self::NoIssuer => ChangeTrustResultCode::NoIssuer,
            Self::InvalidLimit => ChangeTrustResultCode::InvalidLimit,
            Self::LowReserve => ChangeTrustResultCode::LowReserve,
            Self::SelfNotAllowed => ChangeTrustResultCode::SelfNotAllowed,
            Self::TrustLineMissing => ChangeTrustResultCode::TrustLineMissing,
            Self::CannotDelete => ChangeTrustResultCode::CannotDelete,
            Self::NotAuthMaintainLiabilities => ChangeTrustResultCode::NotAuthMaintainLiabilities,
        }
    }

    #[must_use]
    pub const fn variants() -> [ChangeTrustResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ChangeTrustResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ChangeTrustResultCode> for ChangeTrustResult {
    #[must_use]
    fn discriminant(&self) -> ChangeTrustResultCode {
        Self::discriminant(self)
    }
}

impl Variants<ChangeTrustResultCode> for ChangeTrustResult {
    fn variants() -> slice::Iter<'static, ChangeTrustResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<ChangeTrustResultCode> for ChangeTrustResult {}

impl ReadXdr for ChangeTrustResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ChangeTrustResultCode = <ChangeTrustResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ChangeTrustResultCode::Success => Self::Success,
                ChangeTrustResultCode::Malformed => Self::Malformed,
                ChangeTrustResultCode::NoIssuer => Self::NoIssuer,
                ChangeTrustResultCode::InvalidLimit => Self::InvalidLimit,
                ChangeTrustResultCode::LowReserve => Self::LowReserve,
                ChangeTrustResultCode::SelfNotAllowed => Self::SelfNotAllowed,
                ChangeTrustResultCode::TrustLineMissing => Self::TrustLineMissing,
                ChangeTrustResultCode::CannotDelete => Self::CannotDelete,
                ChangeTrustResultCode::NotAuthMaintainLiabilities => {
                    Self::NotAuthMaintainLiabilities
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ChangeTrustResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::NoIssuer => ().write_xdr(w)?,
                Self::InvalidLimit => ().write_xdr(w)?,
                Self::LowReserve => ().write_xdr(w)?,
                Self::SelfNotAllowed => ().write_xdr(w)?,
                Self::TrustLineMissing => ().write_xdr(w)?,
                Self::CannotDelete => ().write_xdr(w)?,
                Self::NotAuthMaintainLiabilities => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ChangeTrustResult`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyChangeTrustResult(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyChangeTrustResult {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyChangeTrustResult {
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
impl LazyXdr for LazyChangeTrustResult {
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
            -5 => {
                // void — no additional data
            }
            -6 => {
                // void — no additional data
            }
            -7 => {
                // void — no additional data
            }
            -8 => {
                // void — no additional data
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
            -5 => {
                // void
            }
            -6 => {
                // void
            }
            -7 => {
                // void
            }
            -8 => {
                // void
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
impl From<LazyHandle> for LazyChangeTrustResult {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyChangeTrustResult {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyChangeTrustResult {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyChangeTrustResult {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> ChangeTrustResultCode {
        // Validated — unwrap is safe.
        ChangeTrustResultCode::try_from(self.discriminant_i32()).unwrap()
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ChangeTrustResult> for LazyChangeTrustResult {
    type Error = Error;
    fn try_from(val: &ChangeTrustResult) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyChangeTrustResult> for ChangeTrustResult {
    type Error = Error;
    fn try_from(lazy: &LazyChangeTrustResult) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
