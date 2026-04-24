#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// CreateClaimableBalanceResult is an XDR Union defined as:
///
/// ```text
/// union CreateClaimableBalanceResult switch (
///     CreateClaimableBalanceResultCode code)
/// {
/// case CREATE_CLAIMABLE_BALANCE_SUCCESS:
///     ClaimableBalanceID balanceID;
/// case CREATE_CLAIMABLE_BALANCE_MALFORMED:
/// case CREATE_CLAIMABLE_BALANCE_LOW_RESERVE:
/// case CREATE_CLAIMABLE_BALANCE_NO_TRUST:
/// case CREATE_CLAIMABLE_BALANCE_NOT_AUTHORIZED:
/// case CREATE_CLAIMABLE_BALANCE_UNDERFUNDED:
///     void;
/// };
/// ```
///
// union with discriminant CreateClaimableBalanceResultCode
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
pub enum CreateClaimableBalanceResult {
    Success(ClaimableBalanceId),
    Malformed,
    LowReserve,
    NoTrust,
    NotAuthorized,
    Underfunded,
}

#[cfg(feature = "alloc")]
impl Default for CreateClaimableBalanceResult {
    fn default() -> Self {
        Self::Success(ClaimableBalanceId::default())
    }
}

impl CreateClaimableBalanceResult {
    const _VARIANTS: &[CreateClaimableBalanceResultCode] = &[
        CreateClaimableBalanceResultCode::Success,
        CreateClaimableBalanceResultCode::Malformed,
        CreateClaimableBalanceResultCode::LowReserve,
        CreateClaimableBalanceResultCode::NoTrust,
        CreateClaimableBalanceResultCode::NotAuthorized,
        CreateClaimableBalanceResultCode::Underfunded,
    ];
    pub const VARIANTS: [CreateClaimableBalanceResultCode; Self::_VARIANTS.len()] = {
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
        "LowReserve",
        "NoTrust",
        "NotAuthorized",
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
            Self::Success(_) => "Success",
            Self::Malformed => "Malformed",
            Self::LowReserve => "LowReserve",
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
            Self::Underfunded => "Underfunded",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> CreateClaimableBalanceResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => CreateClaimableBalanceResultCode::Success,
            Self::Malformed => CreateClaimableBalanceResultCode::Malformed,
            Self::LowReserve => CreateClaimableBalanceResultCode::LowReserve,
            Self::NoTrust => CreateClaimableBalanceResultCode::NoTrust,
            Self::NotAuthorized => CreateClaimableBalanceResultCode::NotAuthorized,
            Self::Underfunded => CreateClaimableBalanceResultCode::Underfunded,
        }
    }

    #[must_use]
    pub const fn variants() -> [CreateClaimableBalanceResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for CreateClaimableBalanceResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<CreateClaimableBalanceResultCode> for CreateClaimableBalanceResult {
    #[must_use]
    fn discriminant(&self) -> CreateClaimableBalanceResultCode {
        Self::discriminant(self)
    }
}

impl Variants<CreateClaimableBalanceResultCode> for CreateClaimableBalanceResult {
    fn variants() -> slice::Iter<'static, CreateClaimableBalanceResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<CreateClaimableBalanceResultCode> for CreateClaimableBalanceResult {}

impl ReadXdr for CreateClaimableBalanceResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: CreateClaimableBalanceResultCode =
                <CreateClaimableBalanceResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                CreateClaimableBalanceResultCode::Success => {
                    Self::Success(ClaimableBalanceId::read_xdr(r)?)
                }
                CreateClaimableBalanceResultCode::Malformed => Self::Malformed,
                CreateClaimableBalanceResultCode::LowReserve => Self::LowReserve,
                CreateClaimableBalanceResultCode::NoTrust => Self::NoTrust,
                CreateClaimableBalanceResultCode::NotAuthorized => Self::NotAuthorized,
                CreateClaimableBalanceResultCode::Underfunded => Self::Underfunded,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for CreateClaimableBalanceResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success(v) => v.write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::LowReserve => ().write_xdr(w)?,
                Self::NoTrust => ().write_xdr(w)?,
                Self::NotAuthorized => ().write_xdr(w)?,
                Self::Underfunded => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`CreateClaimableBalanceResult`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyCreateClaimableBalanceResult(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyCreateClaimableBalanceResult {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyCreateClaimableBalanceResult {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_success().cmp(&other.as_success()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyCreateClaimableBalanceResult {
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
                let field_len =
                    <LazyClaimableBalanceId as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
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
        let bytes: [u8; 4] = buf[..4].try_into().unwrap();
        let disc = i32::from_be_bytes(bytes);
        #[allow(unused_mut)]
        let mut pos: u32 = 4;
        #[allow(clippy::match_same_arms)]
        match disc {
            0 => {
                pos += <LazyClaimableBalanceId as LazyXdr>::xdr_len(&buf[pos as usize..]);
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

    fn from_xdr_consume(parent: &LazyHandle, buf: &mut &[u8]) -> Self {
        let len = Self::xdr_len(buf);
        let offset = (parent.len() as usize - buf.len()) as u32;
        let handle = parent.sub_handle(offset, len);
        *buf = &buf[len as usize..];
        Self(handle)
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyCreateClaimableBalanceResult {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyCreateClaimableBalanceResult {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyCreateClaimableBalanceResult {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyCreateClaimableBalanceResult {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> CreateClaimableBalanceResultCode {
        // Validated — unwrap is safe.
        CreateClaimableBalanceResultCode::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `Success`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_success(&self) -> Option<LazyClaimableBalanceId> {
        if self.discriminant_i32() == 0 {
            Some(<LazyClaimableBalanceId as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&CreateClaimableBalanceResult> for LazyCreateClaimableBalanceResult {
    type Error = Error;
    fn try_from(val: &CreateClaimableBalanceResult) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyCreateClaimableBalanceResult> for CreateClaimableBalanceResult {
    type Error = Error;
    fn try_from(lazy: &LazyCreateClaimableBalanceResult) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
