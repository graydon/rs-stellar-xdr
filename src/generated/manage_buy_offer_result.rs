#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ManageBuyOfferResult is an XDR Union defined as:
///
/// ```text
/// union ManageBuyOfferResult switch (ManageBuyOfferResultCode code)
/// {
/// case MANAGE_BUY_OFFER_SUCCESS:
///     ManageOfferSuccessResult success;
/// case MANAGE_BUY_OFFER_MALFORMED:
/// case MANAGE_BUY_OFFER_SELL_NO_TRUST:
/// case MANAGE_BUY_OFFER_BUY_NO_TRUST:
/// case MANAGE_BUY_OFFER_SELL_NOT_AUTHORIZED:
/// case MANAGE_BUY_OFFER_BUY_NOT_AUTHORIZED:
/// case MANAGE_BUY_OFFER_LINE_FULL:
/// case MANAGE_BUY_OFFER_UNDERFUNDED:
/// case MANAGE_BUY_OFFER_CROSS_SELF:
/// case MANAGE_BUY_OFFER_SELL_NO_ISSUER:
/// case MANAGE_BUY_OFFER_BUY_NO_ISSUER:
/// case MANAGE_BUY_OFFER_NOT_FOUND:
/// case MANAGE_BUY_OFFER_LOW_RESERVE:
///     void;
/// };
/// ```
///
// union with discriminant ManageBuyOfferResultCode
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
pub enum ManageBuyOfferResult {
    Success(ManageOfferSuccessResult),
    Malformed,
    SellNoTrust,
    BuyNoTrust,
    SellNotAuthorized,
    BuyNotAuthorized,
    LineFull,
    Underfunded,
    CrossSelf,
    SellNoIssuer,
    BuyNoIssuer,
    NotFound,
    LowReserve,
}

#[cfg(feature = "alloc")]
impl Default for ManageBuyOfferResult {
    fn default() -> Self {
        Self::Success(ManageOfferSuccessResult::default())
    }
}

impl ManageBuyOfferResult {
    const _VARIANTS: &[ManageBuyOfferResultCode] = &[
        ManageBuyOfferResultCode::Success,
        ManageBuyOfferResultCode::Malformed,
        ManageBuyOfferResultCode::SellNoTrust,
        ManageBuyOfferResultCode::BuyNoTrust,
        ManageBuyOfferResultCode::SellNotAuthorized,
        ManageBuyOfferResultCode::BuyNotAuthorized,
        ManageBuyOfferResultCode::LineFull,
        ManageBuyOfferResultCode::Underfunded,
        ManageBuyOfferResultCode::CrossSelf,
        ManageBuyOfferResultCode::SellNoIssuer,
        ManageBuyOfferResultCode::BuyNoIssuer,
        ManageBuyOfferResultCode::NotFound,
        ManageBuyOfferResultCode::LowReserve,
    ];
    pub const VARIANTS: [ManageBuyOfferResultCode; Self::_VARIANTS.len()] = {
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
        "SellNoTrust",
        "BuyNoTrust",
        "SellNotAuthorized",
        "BuyNotAuthorized",
        "LineFull",
        "Underfunded",
        "CrossSelf",
        "SellNoIssuer",
        "BuyNoIssuer",
        "NotFound",
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
            Self::Success(_) => "Success",
            Self::Malformed => "Malformed",
            Self::SellNoTrust => "SellNoTrust",
            Self::BuyNoTrust => "BuyNoTrust",
            Self::SellNotAuthorized => "SellNotAuthorized",
            Self::BuyNotAuthorized => "BuyNotAuthorized",
            Self::LineFull => "LineFull",
            Self::Underfunded => "Underfunded",
            Self::CrossSelf => "CrossSelf",
            Self::SellNoIssuer => "SellNoIssuer",
            Self::BuyNoIssuer => "BuyNoIssuer",
            Self::NotFound => "NotFound",
            Self::LowReserve => "LowReserve",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ManageBuyOfferResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => ManageBuyOfferResultCode::Success,
            Self::Malformed => ManageBuyOfferResultCode::Malformed,
            Self::SellNoTrust => ManageBuyOfferResultCode::SellNoTrust,
            Self::BuyNoTrust => ManageBuyOfferResultCode::BuyNoTrust,
            Self::SellNotAuthorized => ManageBuyOfferResultCode::SellNotAuthorized,
            Self::BuyNotAuthorized => ManageBuyOfferResultCode::BuyNotAuthorized,
            Self::LineFull => ManageBuyOfferResultCode::LineFull,
            Self::Underfunded => ManageBuyOfferResultCode::Underfunded,
            Self::CrossSelf => ManageBuyOfferResultCode::CrossSelf,
            Self::SellNoIssuer => ManageBuyOfferResultCode::SellNoIssuer,
            Self::BuyNoIssuer => ManageBuyOfferResultCode::BuyNoIssuer,
            Self::NotFound => ManageBuyOfferResultCode::NotFound,
            Self::LowReserve => ManageBuyOfferResultCode::LowReserve,
        }
    }

    #[must_use]
    pub const fn variants() -> [ManageBuyOfferResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ManageBuyOfferResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ManageBuyOfferResultCode> for ManageBuyOfferResult {
    #[must_use]
    fn discriminant(&self) -> ManageBuyOfferResultCode {
        Self::discriminant(self)
    }
}

impl Variants<ManageBuyOfferResultCode> for ManageBuyOfferResult {
    fn variants() -> slice::Iter<'static, ManageBuyOfferResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<ManageBuyOfferResultCode> for ManageBuyOfferResult {}

impl ReadXdr for ManageBuyOfferResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ManageBuyOfferResultCode = <ManageBuyOfferResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ManageBuyOfferResultCode::Success => {
                    Self::Success(ManageOfferSuccessResult::read_xdr(r)?)
                }
                ManageBuyOfferResultCode::Malformed => Self::Malformed,
                ManageBuyOfferResultCode::SellNoTrust => Self::SellNoTrust,
                ManageBuyOfferResultCode::BuyNoTrust => Self::BuyNoTrust,
                ManageBuyOfferResultCode::SellNotAuthorized => Self::SellNotAuthorized,
                ManageBuyOfferResultCode::BuyNotAuthorized => Self::BuyNotAuthorized,
                ManageBuyOfferResultCode::LineFull => Self::LineFull,
                ManageBuyOfferResultCode::Underfunded => Self::Underfunded,
                ManageBuyOfferResultCode::CrossSelf => Self::CrossSelf,
                ManageBuyOfferResultCode::SellNoIssuer => Self::SellNoIssuer,
                ManageBuyOfferResultCode::BuyNoIssuer => Self::BuyNoIssuer,
                ManageBuyOfferResultCode::NotFound => Self::NotFound,
                ManageBuyOfferResultCode::LowReserve => Self::LowReserve,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ManageBuyOfferResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success(v) => v.write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::SellNoTrust => ().write_xdr(w)?,
                Self::BuyNoTrust => ().write_xdr(w)?,
                Self::SellNotAuthorized => ().write_xdr(w)?,
                Self::BuyNotAuthorized => ().write_xdr(w)?,
                Self::LineFull => ().write_xdr(w)?,
                Self::Underfunded => ().write_xdr(w)?,
                Self::CrossSelf => ().write_xdr(w)?,
                Self::SellNoIssuer => ().write_xdr(w)?,
                Self::BuyNoIssuer => ().write_xdr(w)?,
                Self::NotFound => ().write_xdr(w)?,
                Self::LowReserve => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ManageBuyOfferResult`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyManageBuyOfferResult(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyManageBuyOfferResult {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyManageBuyOfferResult {
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
impl LazyXdr for LazyManageBuyOfferResult {
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
                let field_len = <LazyManageOfferSuccessResult as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
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
            -11 => {
                // void — no additional data
            }
            -12 => {
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
                pos += <LazyManageOfferSuccessResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
            -11 => {
                // void
            }
            -12 => {
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
impl From<LazyHandle> for LazyManageBuyOfferResult {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyManageBuyOfferResult {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyManageBuyOfferResult {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyManageBuyOfferResult {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> ManageBuyOfferResultCode {
        // Validated — unwrap is safe.
        ManageBuyOfferResultCode::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `Success`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_success(&self) -> Option<LazyManageOfferSuccessResult> {
        if self.discriminant_i32() == 0 {
            Some(<LazyManageOfferSuccessResult as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ManageBuyOfferResult> for LazyManageBuyOfferResult {
    type Error = Error;
    fn try_from(val: &ManageBuyOfferResult) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyManageBuyOfferResult> for ManageBuyOfferResult {
    type Error = Error;
    fn try_from(lazy: &LazyManageBuyOfferResult) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
