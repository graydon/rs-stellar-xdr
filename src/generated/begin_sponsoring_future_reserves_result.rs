#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// BeginSponsoringFutureReservesResult is an XDR Union defined as:
///
/// ```text
/// union BeginSponsoringFutureReservesResult switch (
///     BeginSponsoringFutureReservesResultCode code)
/// {
/// case BEGIN_SPONSORING_FUTURE_RESERVES_SUCCESS:
///     void;
/// case BEGIN_SPONSORING_FUTURE_RESERVES_MALFORMED:
/// case BEGIN_SPONSORING_FUTURE_RESERVES_ALREADY_SPONSORED:
/// case BEGIN_SPONSORING_FUTURE_RESERVES_RECURSIVE:
///     void;
/// };
/// ```
///
// union with discriminant BeginSponsoringFutureReservesResultCode
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
pub enum BeginSponsoringFutureReservesResult {
    Success,
    Malformed,
    AlreadySponsored,
    Recursive,
}

#[cfg(feature = "alloc")]
impl Default for BeginSponsoringFutureReservesResult {
    fn default() -> Self {
        Self::Success
    }
}

impl BeginSponsoringFutureReservesResult {
    const _VARIANTS: &[BeginSponsoringFutureReservesResultCode] = &[
        BeginSponsoringFutureReservesResultCode::Success,
        BeginSponsoringFutureReservesResultCode::Malformed,
        BeginSponsoringFutureReservesResultCode::AlreadySponsored,
        BeginSponsoringFutureReservesResultCode::Recursive,
    ];
    pub const VARIANTS: [BeginSponsoringFutureReservesResultCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Success", "Malformed", "AlreadySponsored", "Recursive"];
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
            Self::AlreadySponsored => "AlreadySponsored",
            Self::Recursive => "Recursive",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> BeginSponsoringFutureReservesResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => BeginSponsoringFutureReservesResultCode::Success,
            Self::Malformed => BeginSponsoringFutureReservesResultCode::Malformed,
            Self::AlreadySponsored => BeginSponsoringFutureReservesResultCode::AlreadySponsored,
            Self::Recursive => BeginSponsoringFutureReservesResultCode::Recursive,
        }
    }

    #[must_use]
    pub const fn variants() -> [BeginSponsoringFutureReservesResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for BeginSponsoringFutureReservesResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<BeginSponsoringFutureReservesResultCode> for BeginSponsoringFutureReservesResult {
    #[must_use]
    fn discriminant(&self) -> BeginSponsoringFutureReservesResultCode {
        Self::discriminant(self)
    }
}

impl Variants<BeginSponsoringFutureReservesResultCode> for BeginSponsoringFutureReservesResult {
    fn variants() -> slice::Iter<'static, BeginSponsoringFutureReservesResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<BeginSponsoringFutureReservesResultCode> for BeginSponsoringFutureReservesResult {}

impl ReadXdr for BeginSponsoringFutureReservesResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: BeginSponsoringFutureReservesResultCode =
                <BeginSponsoringFutureReservesResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                BeginSponsoringFutureReservesResultCode::Success => Self::Success,
                BeginSponsoringFutureReservesResultCode::Malformed => Self::Malformed,
                BeginSponsoringFutureReservesResultCode::AlreadySponsored => Self::AlreadySponsored,
                BeginSponsoringFutureReservesResultCode::Recursive => Self::Recursive,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for BeginSponsoringFutureReservesResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::AlreadySponsored => ().write_xdr(w)?,
                Self::Recursive => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`BeginSponsoringFutureReservesResult`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyBeginSponsoringFutureReservesResult(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyBeginSponsoringFutureReservesResult {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyBeginSponsoringFutureReservesResult {
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
impl LazyXdr for LazyBeginSponsoringFutureReservesResult {
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
impl From<LazyHandle> for LazyBeginSponsoringFutureReservesResult {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyBeginSponsoringFutureReservesResult {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyBeginSponsoringFutureReservesResult {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyBeginSponsoringFutureReservesResult {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> BeginSponsoringFutureReservesResultCode {
        // Validated — unwrap is safe.
        BeginSponsoringFutureReservesResultCode::try_from(self.discriminant_i32()).unwrap()
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&BeginSponsoringFutureReservesResult> for LazyBeginSponsoringFutureReservesResult {
    type Error = Error;
    fn try_from(val: &BeginSponsoringFutureReservesResult) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyBeginSponsoringFutureReservesResult> for BeginSponsoringFutureReservesResult {
    type Error = Error;
    fn try_from(lazy: &LazyBeginSponsoringFutureReservesResult) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
