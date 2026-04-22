#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ManageDataResult is an XDR Union defined as:
///
/// ```text
/// union ManageDataResult switch (ManageDataResultCode code)
/// {
/// case MANAGE_DATA_SUCCESS:
///     void;
/// case MANAGE_DATA_NOT_SUPPORTED_YET:
/// case MANAGE_DATA_NAME_NOT_FOUND:
/// case MANAGE_DATA_LOW_RESERVE:
/// case MANAGE_DATA_INVALID_NAME:
///     void;
/// };
/// ```
///
// union with discriminant ManageDataResultCode
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
pub enum ManageDataResult {
    Success,
    NotSupportedYet,
    NameNotFound,
    LowReserve,
    InvalidName,
}

#[cfg(feature = "alloc")]
impl Default for ManageDataResult {
    fn default() -> Self {
        Self::Success
    }
}

impl ManageDataResult {
    const _VARIANTS: &[ManageDataResultCode] = &[
        ManageDataResultCode::Success,
        ManageDataResultCode::NotSupportedYet,
        ManageDataResultCode::NameNotFound,
        ManageDataResultCode::LowReserve,
        ManageDataResultCode::InvalidName,
    ];
    pub const VARIANTS: [ManageDataResultCode; Self::_VARIANTS.len()] = {
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
        "NotSupportedYet",
        "NameNotFound",
        "LowReserve",
        "InvalidName",
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
            Self::NotSupportedYet => "NotSupportedYet",
            Self::NameNotFound => "NameNotFound",
            Self::LowReserve => "LowReserve",
            Self::InvalidName => "InvalidName",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ManageDataResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ManageDataResultCode::Success,
            Self::NotSupportedYet => ManageDataResultCode::NotSupportedYet,
            Self::NameNotFound => ManageDataResultCode::NameNotFound,
            Self::LowReserve => ManageDataResultCode::LowReserve,
            Self::InvalidName => ManageDataResultCode::InvalidName,
        }
    }

    #[must_use]
    pub const fn variants() -> [ManageDataResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ManageDataResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ManageDataResultCode> for ManageDataResult {
    #[must_use]
    fn discriminant(&self) -> ManageDataResultCode {
        Self::discriminant(self)
    }
}

impl Variants<ManageDataResultCode> for ManageDataResult {
    fn variants() -> slice::Iter<'static, ManageDataResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<ManageDataResultCode> for ManageDataResult {}

impl ReadXdr for ManageDataResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ManageDataResultCode = <ManageDataResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ManageDataResultCode::Success => Self::Success,
                ManageDataResultCode::NotSupportedYet => Self::NotSupportedYet,
                ManageDataResultCode::NameNotFound => Self::NameNotFound,
                ManageDataResultCode::LowReserve => Self::LowReserve,
                ManageDataResultCode::InvalidName => Self::InvalidName,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ManageDataResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::NotSupportedYet => ().write_xdr(w)?,
                Self::NameNotFound => ().write_xdr(w)?,
                Self::LowReserve => ().write_xdr(w)?,
                Self::InvalidName => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ManageDataResult`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyManageDataResult(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyManageDataResult {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyManageDataResult {
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
impl LazyXdr for LazyManageDataResult {
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
impl From<LazyHandle> for LazyManageDataResult {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyManageDataResult {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyManageDataResult {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyManageDataResult {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> ManageDataResultCode {
        // Validated — unwrap is safe.
        ManageDataResultCode::try_from(self.discriminant_i32()).unwrap()
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ManageDataResult> for LazyManageDataResult {
    type Error = Error;
    fn try_from(val: &ManageDataResult) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyManageDataResult> for ManageDataResult {
    type Error = Error;
    fn try_from(lazy: &LazyManageDataResult) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
