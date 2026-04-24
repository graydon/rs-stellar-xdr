#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SetOptionsResult is an XDR Union defined as:
///
/// ```text
/// union SetOptionsResult switch (SetOptionsResultCode code)
/// {
/// case SET_OPTIONS_SUCCESS:
///     void;
/// case SET_OPTIONS_LOW_RESERVE:
/// case SET_OPTIONS_TOO_MANY_SIGNERS:
/// case SET_OPTIONS_BAD_FLAGS:
/// case SET_OPTIONS_INVALID_INFLATION:
/// case SET_OPTIONS_CANT_CHANGE:
/// case SET_OPTIONS_UNKNOWN_FLAG:
/// case SET_OPTIONS_THRESHOLD_OUT_OF_RANGE:
/// case SET_OPTIONS_BAD_SIGNER:
/// case SET_OPTIONS_INVALID_HOME_DOMAIN:
/// case SET_OPTIONS_AUTH_REVOCABLE_REQUIRED:
///     void;
/// };
/// ```
///
// union with discriminant SetOptionsResultCode
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
pub enum SetOptionsResult {
    Success,
    LowReserve,
    TooManySigners,
    BadFlags,
    InvalidInflation,
    CantChange,
    UnknownFlag,
    ThresholdOutOfRange,
    BadSigner,
    InvalidHomeDomain,
    AuthRevocableRequired,
}

#[cfg(feature = "alloc")]
impl Default for SetOptionsResult {
    fn default() -> Self {
        Self::Success
    }
}

impl SetOptionsResult {
    const _VARIANTS: &[SetOptionsResultCode] = &[
        SetOptionsResultCode::Success,
        SetOptionsResultCode::LowReserve,
        SetOptionsResultCode::TooManySigners,
        SetOptionsResultCode::BadFlags,
        SetOptionsResultCode::InvalidInflation,
        SetOptionsResultCode::CantChange,
        SetOptionsResultCode::UnknownFlag,
        SetOptionsResultCode::ThresholdOutOfRange,
        SetOptionsResultCode::BadSigner,
        SetOptionsResultCode::InvalidHomeDomain,
        SetOptionsResultCode::AuthRevocableRequired,
    ];
    pub const VARIANTS: [SetOptionsResultCode; Self::_VARIANTS.len()] = {
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
        "LowReserve",
        "TooManySigners",
        "BadFlags",
        "InvalidInflation",
        "CantChange",
        "UnknownFlag",
        "ThresholdOutOfRange",
        "BadSigner",
        "InvalidHomeDomain",
        "AuthRevocableRequired",
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
            Self::LowReserve => "LowReserve",
            Self::TooManySigners => "TooManySigners",
            Self::BadFlags => "BadFlags",
            Self::InvalidInflation => "InvalidInflation",
            Self::CantChange => "CantChange",
            Self::UnknownFlag => "UnknownFlag",
            Self::ThresholdOutOfRange => "ThresholdOutOfRange",
            Self::BadSigner => "BadSigner",
            Self::InvalidHomeDomain => "InvalidHomeDomain",
            Self::AuthRevocableRequired => "AuthRevocableRequired",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> SetOptionsResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => SetOptionsResultCode::Success,
            Self::LowReserve => SetOptionsResultCode::LowReserve,
            Self::TooManySigners => SetOptionsResultCode::TooManySigners,
            Self::BadFlags => SetOptionsResultCode::BadFlags,
            Self::InvalidInflation => SetOptionsResultCode::InvalidInflation,
            Self::CantChange => SetOptionsResultCode::CantChange,
            Self::UnknownFlag => SetOptionsResultCode::UnknownFlag,
            Self::ThresholdOutOfRange => SetOptionsResultCode::ThresholdOutOfRange,
            Self::BadSigner => SetOptionsResultCode::BadSigner,
            Self::InvalidHomeDomain => SetOptionsResultCode::InvalidHomeDomain,
            Self::AuthRevocableRequired => SetOptionsResultCode::AuthRevocableRequired,
        }
    }

    #[must_use]
    pub const fn variants() -> [SetOptionsResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for SetOptionsResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<SetOptionsResultCode> for SetOptionsResult {
    #[must_use]
    fn discriminant(&self) -> SetOptionsResultCode {
        Self::discriminant(self)
    }
}

impl Variants<SetOptionsResultCode> for SetOptionsResult {
    fn variants() -> slice::Iter<'static, SetOptionsResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<SetOptionsResultCode> for SetOptionsResult {}

impl ReadXdr for SetOptionsResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: SetOptionsResultCode = <SetOptionsResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                SetOptionsResultCode::Success => Self::Success,
                SetOptionsResultCode::LowReserve => Self::LowReserve,
                SetOptionsResultCode::TooManySigners => Self::TooManySigners,
                SetOptionsResultCode::BadFlags => Self::BadFlags,
                SetOptionsResultCode::InvalidInflation => Self::InvalidInflation,
                SetOptionsResultCode::CantChange => Self::CantChange,
                SetOptionsResultCode::UnknownFlag => Self::UnknownFlag,
                SetOptionsResultCode::ThresholdOutOfRange => Self::ThresholdOutOfRange,
                SetOptionsResultCode::BadSigner => Self::BadSigner,
                SetOptionsResultCode::InvalidHomeDomain => Self::InvalidHomeDomain,
                SetOptionsResultCode::AuthRevocableRequired => Self::AuthRevocableRequired,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for SetOptionsResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::LowReserve => ().write_xdr(w)?,
                Self::TooManySigners => ().write_xdr(w)?,
                Self::BadFlags => ().write_xdr(w)?,
                Self::InvalidInflation => ().write_xdr(w)?,
                Self::CantChange => ().write_xdr(w)?,
                Self::UnknownFlag => ().write_xdr(w)?,
                Self::ThresholdOutOfRange => ().write_xdr(w)?,
                Self::BadSigner => ().write_xdr(w)?,
                Self::InvalidHomeDomain => ().write_xdr(w)?,
                Self::AuthRevocableRequired => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`SetOptionsResult`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySetOptionsResult(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazySetOptionsResult {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazySetOptionsResult {
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
impl LazyXdr for LazySetOptionsResult {
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
            -9 => {
                // void — no additional data
            }
            -10 => {
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
            -9 => {
                // void
            }
            -10 => {
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
impl From<LazyHandle> for LazySetOptionsResult {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazySetOptionsResult {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazySetOptionsResult {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazySetOptionsResult {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> SetOptionsResultCode {
        // Validated — unwrap is safe.
        SetOptionsResultCode::try_from(self.discriminant_i32()).unwrap()
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&SetOptionsResult> for LazySetOptionsResult {
    type Error = Error;
    fn try_from(val: &SetOptionsResult) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazySetOptionsResult> for SetOptionsResult {
    type Error = Error;
    fn try_from(lazy: &LazySetOptionsResult) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
