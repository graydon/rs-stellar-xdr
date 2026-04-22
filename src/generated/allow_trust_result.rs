#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// AllowTrustResult is an XDR Union defined as:
///
/// ```text
/// union AllowTrustResult switch (AllowTrustResultCode code)
/// {
/// case ALLOW_TRUST_SUCCESS:
///     void;
/// case ALLOW_TRUST_MALFORMED:
/// case ALLOW_TRUST_NO_TRUST_LINE:
/// case ALLOW_TRUST_TRUST_NOT_REQUIRED:
/// case ALLOW_TRUST_CANT_REVOKE:
/// case ALLOW_TRUST_SELF_NOT_ALLOWED:
/// case ALLOW_TRUST_LOW_RESERVE:
///     void;
/// };
/// ```
///
// union with discriminant AllowTrustResultCode
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
pub enum AllowTrustResult {
    Success,
    Malformed,
    NoTrustLine,
    TrustNotRequired,
    CantRevoke,
    SelfNotAllowed,
    LowReserve,
}

#[cfg(feature = "alloc")]
impl Default for AllowTrustResult {
    fn default() -> Self {
        Self::Success
    }
}

impl AllowTrustResult {
    const _VARIANTS: &[AllowTrustResultCode] = &[
        AllowTrustResultCode::Success,
        AllowTrustResultCode::Malformed,
        AllowTrustResultCode::NoTrustLine,
        AllowTrustResultCode::TrustNotRequired,
        AllowTrustResultCode::CantRevoke,
        AllowTrustResultCode::SelfNotAllowed,
        AllowTrustResultCode::LowReserve,
    ];
    pub const VARIANTS: [AllowTrustResultCode; Self::_VARIANTS.len()] = {
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
        "NoTrustLine",
        "TrustNotRequired",
        "CantRevoke",
        "SelfNotAllowed",
        "LowReserve",
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
            Self::NoTrustLine => "NoTrustLine",
            Self::TrustNotRequired => "TrustNotRequired",
            Self::CantRevoke => "CantRevoke",
            Self::SelfNotAllowed => "SelfNotAllowed",
            Self::LowReserve => "LowReserve",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> AllowTrustResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => AllowTrustResultCode::Success,
            Self::Malformed => AllowTrustResultCode::Malformed,
            Self::NoTrustLine => AllowTrustResultCode::NoTrustLine,
            Self::TrustNotRequired => AllowTrustResultCode::TrustNotRequired,
            Self::CantRevoke => AllowTrustResultCode::CantRevoke,
            Self::SelfNotAllowed => AllowTrustResultCode::SelfNotAllowed,
            Self::LowReserve => AllowTrustResultCode::LowReserve,
        }
    }

    #[must_use]
    pub const fn variants() -> [AllowTrustResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for AllowTrustResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<AllowTrustResultCode> for AllowTrustResult {
    #[must_use]
    fn discriminant(&self) -> AllowTrustResultCode {
        Self::discriminant(self)
    }
}

impl Variants<AllowTrustResultCode> for AllowTrustResult {
    fn variants() -> slice::Iter<'static, AllowTrustResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<AllowTrustResultCode> for AllowTrustResult {}

impl ReadXdr for AllowTrustResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: AllowTrustResultCode = <AllowTrustResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                AllowTrustResultCode::Success => Self::Success,
                AllowTrustResultCode::Malformed => Self::Malformed,
                AllowTrustResultCode::NoTrustLine => Self::NoTrustLine,
                AllowTrustResultCode::TrustNotRequired => Self::TrustNotRequired,
                AllowTrustResultCode::CantRevoke => Self::CantRevoke,
                AllowTrustResultCode::SelfNotAllowed => Self::SelfNotAllowed,
                AllowTrustResultCode::LowReserve => Self::LowReserve,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for AllowTrustResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::NoTrustLine => ().write_xdr(w)?,
                Self::TrustNotRequired => ().write_xdr(w)?,
                Self::CantRevoke => ().write_xdr(w)?,
                Self::SelfNotAllowed => ().write_xdr(w)?,
                Self::LowReserve => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`AllowTrustResult`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyAllowTrustResult(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyAllowTrustResult {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyAllowTrustResult {
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
impl LazyXdr for LazyAllowTrustResult {
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
impl From<LazyHandle> for LazyAllowTrustResult {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyAllowTrustResult {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyAllowTrustResult {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyAllowTrustResult {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> AllowTrustResultCode {
        // Validated — unwrap is safe.
        AllowTrustResultCode::try_from(self.discriminant_i32()).unwrap()
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&AllowTrustResult> for LazyAllowTrustResult {
    type Error = Error;
    fn try_from(val: &AllowTrustResult) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyAllowTrustResult> for AllowTrustResult {
    type Error = Error;
    fn try_from(lazy: &LazyAllowTrustResult) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
