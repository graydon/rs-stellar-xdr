#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ClawbackResult is an XDR Union defined as:
///
/// ```text
/// union ClawbackResult switch (ClawbackResultCode code)
/// {
/// case CLAWBACK_SUCCESS:
///     void;
/// case CLAWBACK_MALFORMED:
/// case CLAWBACK_NOT_CLAWBACK_ENABLED:
/// case CLAWBACK_NO_TRUST:
/// case CLAWBACK_UNDERFUNDED:
///     void;
/// };
/// ```
///
// union with discriminant ClawbackResultCode
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
pub enum ClawbackResult {
    Success,
    Malformed,
    NotClawbackEnabled,
    NoTrust,
    Underfunded,
}

#[cfg(feature = "alloc")]
impl Default for ClawbackResult {
    fn default() -> Self {
        Self::Success
    }
}

impl ClawbackResult {
    const _VARIANTS: &[ClawbackResultCode] = &[
        ClawbackResultCode::Success,
        ClawbackResultCode::Malformed,
        ClawbackResultCode::NotClawbackEnabled,
        ClawbackResultCode::NoTrust,
        ClawbackResultCode::Underfunded,
    ];
    pub const VARIANTS: [ClawbackResultCode; Self::_VARIANTS.len()] = {
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
        "NotClawbackEnabled",
        "NoTrust",
        "Underfunded",
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
            Self::NotClawbackEnabled => "NotClawbackEnabled",
            Self::NoTrust => "NoTrust",
            Self::Underfunded => "Underfunded",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ClawbackResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ClawbackResultCode::Success,
            Self::Malformed => ClawbackResultCode::Malformed,
            Self::NotClawbackEnabled => ClawbackResultCode::NotClawbackEnabled,
            Self::NoTrust => ClawbackResultCode::NoTrust,
            Self::Underfunded => ClawbackResultCode::Underfunded,
        }
    }

    #[must_use]
    pub const fn variants() -> [ClawbackResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ClawbackResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ClawbackResultCode> for ClawbackResult {
    #[must_use]
    fn discriminant(&self) -> ClawbackResultCode {
        Self::discriminant(self)
    }
}

impl Variants<ClawbackResultCode> for ClawbackResult {
    fn variants() -> slice::Iter<'static, ClawbackResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<ClawbackResultCode> for ClawbackResult {}

impl ReadXdr for ClawbackResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ClawbackResultCode = <ClawbackResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ClawbackResultCode::Success => Self::Success,
                ClawbackResultCode::Malformed => Self::Malformed,
                ClawbackResultCode::NotClawbackEnabled => Self::NotClawbackEnabled,
                ClawbackResultCode::NoTrust => Self::NoTrust,
                ClawbackResultCode::Underfunded => Self::Underfunded,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ClawbackResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::NotClawbackEnabled => ().write_xdr(w)?,
                Self::NoTrust => ().write_xdr(w)?,
                Self::Underfunded => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ClawbackResult`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyClawbackResult(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyClawbackResult {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyClawbackResult {
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
impl LazyXdr for LazyClawbackResult {
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
impl From<LazyHandle> for LazyClawbackResult {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyClawbackResult {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyClawbackResult {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyClawbackResult {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> ClawbackResultCode {
        // Validated — unwrap is safe.
        ClawbackResultCode::try_from(self.discriminant_i32()).unwrap()
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ClawbackResult> for LazyClawbackResult {
    type Error = Error;
    fn try_from(val: &ClawbackResult) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyClawbackResult> for ClawbackResult {
    type Error = Error;
    fn try_from(lazy: &LazyClawbackResult) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
