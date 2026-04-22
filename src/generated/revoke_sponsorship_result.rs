#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// RevokeSponsorshipResult is an XDR Union defined as:
///
/// ```text
/// union RevokeSponsorshipResult switch (RevokeSponsorshipResultCode code)
/// {
/// case REVOKE_SPONSORSHIP_SUCCESS:
///     void;
/// case REVOKE_SPONSORSHIP_DOES_NOT_EXIST:
/// case REVOKE_SPONSORSHIP_NOT_SPONSOR:
/// case REVOKE_SPONSORSHIP_LOW_RESERVE:
/// case REVOKE_SPONSORSHIP_ONLY_TRANSFERABLE:
/// case REVOKE_SPONSORSHIP_MALFORMED:
///     void;
/// };
/// ```
///
// union with discriminant RevokeSponsorshipResultCode
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
pub enum RevokeSponsorshipResult {
    Success,
    DoesNotExist,
    NotSponsor,
    LowReserve,
    OnlyTransferable,
    Malformed,
}

#[cfg(feature = "alloc")]
impl Default for RevokeSponsorshipResult {
    fn default() -> Self {
        Self::Success
    }
}

impl RevokeSponsorshipResult {
    const _VARIANTS: &[RevokeSponsorshipResultCode] = &[
        RevokeSponsorshipResultCode::Success,
        RevokeSponsorshipResultCode::DoesNotExist,
        RevokeSponsorshipResultCode::NotSponsor,
        RevokeSponsorshipResultCode::LowReserve,
        RevokeSponsorshipResultCode::OnlyTransferable,
        RevokeSponsorshipResultCode::Malformed,
    ];
    pub const VARIANTS: [RevokeSponsorshipResultCode; Self::_VARIANTS.len()] = {
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
        "DoesNotExist",
        "NotSponsor",
        "LowReserve",
        "OnlyTransferable",
        "Malformed",
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
            Self::DoesNotExist => "DoesNotExist",
            Self::NotSponsor => "NotSponsor",
            Self::LowReserve => "LowReserve",
            Self::OnlyTransferable => "OnlyTransferable",
            Self::Malformed => "Malformed",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> RevokeSponsorshipResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => RevokeSponsorshipResultCode::Success,
            Self::DoesNotExist => RevokeSponsorshipResultCode::DoesNotExist,
            Self::NotSponsor => RevokeSponsorshipResultCode::NotSponsor,
            Self::LowReserve => RevokeSponsorshipResultCode::LowReserve,
            Self::OnlyTransferable => RevokeSponsorshipResultCode::OnlyTransferable,
            Self::Malformed => RevokeSponsorshipResultCode::Malformed,
        }
    }

    #[must_use]
    pub const fn variants() -> [RevokeSponsorshipResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for RevokeSponsorshipResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<RevokeSponsorshipResultCode> for RevokeSponsorshipResult {
    #[must_use]
    fn discriminant(&self) -> RevokeSponsorshipResultCode {
        Self::discriminant(self)
    }
}

impl Variants<RevokeSponsorshipResultCode> for RevokeSponsorshipResult {
    fn variants() -> slice::Iter<'static, RevokeSponsorshipResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<RevokeSponsorshipResultCode> for RevokeSponsorshipResult {}

impl ReadXdr for RevokeSponsorshipResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: RevokeSponsorshipResultCode =
                <RevokeSponsorshipResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                RevokeSponsorshipResultCode::Success => Self::Success,
                RevokeSponsorshipResultCode::DoesNotExist => Self::DoesNotExist,
                RevokeSponsorshipResultCode::NotSponsor => Self::NotSponsor,
                RevokeSponsorshipResultCode::LowReserve => Self::LowReserve,
                RevokeSponsorshipResultCode::OnlyTransferable => Self::OnlyTransferable,
                RevokeSponsorshipResultCode::Malformed => Self::Malformed,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for RevokeSponsorshipResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::DoesNotExist => ().write_xdr(w)?,
                Self::NotSponsor => ().write_xdr(w)?,
                Self::LowReserve => ().write_xdr(w)?,
                Self::OnlyTransferable => ().write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`RevokeSponsorshipResult`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyRevokeSponsorshipResult(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyRevokeSponsorshipResult {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyRevokeSponsorshipResult {
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
impl LazyXdr for LazyRevokeSponsorshipResult {
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
impl From<LazyHandle> for LazyRevokeSponsorshipResult {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyRevokeSponsorshipResult {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyRevokeSponsorshipResult {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyRevokeSponsorshipResult {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> RevokeSponsorshipResultCode {
        // Validated — unwrap is safe.
        RevokeSponsorshipResultCode::try_from(self.discriminant_i32()).unwrap()
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&RevokeSponsorshipResult> for LazyRevokeSponsorshipResult {
    type Error = Error;
    fn try_from(val: &RevokeSponsorshipResult) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyRevokeSponsorshipResult> for RevokeSponsorshipResult {
    type Error = Error;
    fn try_from(lazy: &LazyRevokeSponsorshipResult) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
