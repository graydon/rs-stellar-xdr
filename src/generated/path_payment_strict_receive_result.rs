#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// PathPaymentStrictReceiveResult is an XDR Union defined as:
///
/// ```text
/// union PathPaymentStrictReceiveResult switch (
///     PathPaymentStrictReceiveResultCode code)
/// {
/// case PATH_PAYMENT_STRICT_RECEIVE_SUCCESS:
///     struct
///     {
///         ClaimAtom offers<>;
///         SimplePaymentResult last;
///     } success;
/// case PATH_PAYMENT_STRICT_RECEIVE_MALFORMED:
/// case PATH_PAYMENT_STRICT_RECEIVE_UNDERFUNDED:
/// case PATH_PAYMENT_STRICT_RECEIVE_SRC_NO_TRUST:
/// case PATH_PAYMENT_STRICT_RECEIVE_SRC_NOT_AUTHORIZED:
/// case PATH_PAYMENT_STRICT_RECEIVE_NO_DESTINATION:
/// case PATH_PAYMENT_STRICT_RECEIVE_NO_TRUST:
/// case PATH_PAYMENT_STRICT_RECEIVE_NOT_AUTHORIZED:
/// case PATH_PAYMENT_STRICT_RECEIVE_LINE_FULL:
///     void;
/// case PATH_PAYMENT_STRICT_RECEIVE_NO_ISSUER:
///     Asset noIssuer; // the asset that caused the error
/// case PATH_PAYMENT_STRICT_RECEIVE_TOO_FEW_OFFERS:
/// case PATH_PAYMENT_STRICT_RECEIVE_OFFER_CROSS_SELF:
/// case PATH_PAYMENT_STRICT_RECEIVE_OVER_SENDMAX:
///     void;
/// };
/// ```
///
// union with discriminant PathPaymentStrictReceiveResultCode
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
pub enum PathPaymentStrictReceiveResult {
    Success(PathPaymentStrictReceiveResultSuccess),
    Malformed,
    Underfunded,
    SrcNoTrust,
    SrcNotAuthorized,
    NoDestination,
    NoTrust,
    NotAuthorized,
    LineFull,
    NoIssuer(Asset),
    TooFewOffers,
    OfferCrossSelf,
    OverSendmax,
}

#[cfg(feature = "alloc")]
impl Default for PathPaymentStrictReceiveResult {
    fn default() -> Self {
        Self::Success(PathPaymentStrictReceiveResultSuccess::default())
    }
}

impl PathPaymentStrictReceiveResult {
    const _VARIANTS: &[PathPaymentStrictReceiveResultCode] = &[
        PathPaymentStrictReceiveResultCode::Success,
        PathPaymentStrictReceiveResultCode::Malformed,
        PathPaymentStrictReceiveResultCode::Underfunded,
        PathPaymentStrictReceiveResultCode::SrcNoTrust,
        PathPaymentStrictReceiveResultCode::SrcNotAuthorized,
        PathPaymentStrictReceiveResultCode::NoDestination,
        PathPaymentStrictReceiveResultCode::NoTrust,
        PathPaymentStrictReceiveResultCode::NotAuthorized,
        PathPaymentStrictReceiveResultCode::LineFull,
        PathPaymentStrictReceiveResultCode::NoIssuer,
        PathPaymentStrictReceiveResultCode::TooFewOffers,
        PathPaymentStrictReceiveResultCode::OfferCrossSelf,
        PathPaymentStrictReceiveResultCode::OverSendmax,
    ];
    pub const VARIANTS: [PathPaymentStrictReceiveResultCode; Self::_VARIANTS.len()] = {
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
        "SrcNoTrust",
        "SrcNotAuthorized",
        "NoDestination",
        "NoTrust",
        "NotAuthorized",
        "LineFull",
        "NoIssuer",
        "TooFewOffers",
        "OfferCrossSelf",
        "OverSendmax",
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
            Self::Underfunded => "Underfunded",
            Self::SrcNoTrust => "SrcNoTrust",
            Self::SrcNotAuthorized => "SrcNotAuthorized",
            Self::NoDestination => "NoDestination",
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
            Self::LineFull => "LineFull",
            Self::NoIssuer(_) => "NoIssuer",
            Self::TooFewOffers => "TooFewOffers",
            Self::OfferCrossSelf => "OfferCrossSelf",
            Self::OverSendmax => "OverSendmax",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> PathPaymentStrictReceiveResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => PathPaymentStrictReceiveResultCode::Success,
            Self::Malformed => PathPaymentStrictReceiveResultCode::Malformed,
            Self::Underfunded => PathPaymentStrictReceiveResultCode::Underfunded,
            Self::SrcNoTrust => PathPaymentStrictReceiveResultCode::SrcNoTrust,
            Self::SrcNotAuthorized => PathPaymentStrictReceiveResultCode::SrcNotAuthorized,
            Self::NoDestination => PathPaymentStrictReceiveResultCode::NoDestination,
            Self::NoTrust => PathPaymentStrictReceiveResultCode::NoTrust,
            Self::NotAuthorized => PathPaymentStrictReceiveResultCode::NotAuthorized,
            Self::LineFull => PathPaymentStrictReceiveResultCode::LineFull,
            Self::NoIssuer(_) => PathPaymentStrictReceiveResultCode::NoIssuer,
            Self::TooFewOffers => PathPaymentStrictReceiveResultCode::TooFewOffers,
            Self::OfferCrossSelf => PathPaymentStrictReceiveResultCode::OfferCrossSelf,
            Self::OverSendmax => PathPaymentStrictReceiveResultCode::OverSendmax,
        }
    }

    #[must_use]
    pub const fn variants() -> [PathPaymentStrictReceiveResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for PathPaymentStrictReceiveResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<PathPaymentStrictReceiveResultCode> for PathPaymentStrictReceiveResult {
    #[must_use]
    fn discriminant(&self) -> PathPaymentStrictReceiveResultCode {
        Self::discriminant(self)
    }
}

impl Variants<PathPaymentStrictReceiveResultCode> for PathPaymentStrictReceiveResult {
    fn variants() -> slice::Iter<'static, PathPaymentStrictReceiveResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<PathPaymentStrictReceiveResultCode> for PathPaymentStrictReceiveResult {}

impl ReadXdr for PathPaymentStrictReceiveResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: PathPaymentStrictReceiveResultCode =
                <PathPaymentStrictReceiveResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                PathPaymentStrictReceiveResultCode::Success => {
                    Self::Success(PathPaymentStrictReceiveResultSuccess::read_xdr(r)?)
                }
                PathPaymentStrictReceiveResultCode::Malformed => Self::Malformed,
                PathPaymentStrictReceiveResultCode::Underfunded => Self::Underfunded,
                PathPaymentStrictReceiveResultCode::SrcNoTrust => Self::SrcNoTrust,
                PathPaymentStrictReceiveResultCode::SrcNotAuthorized => Self::SrcNotAuthorized,
                PathPaymentStrictReceiveResultCode::NoDestination => Self::NoDestination,
                PathPaymentStrictReceiveResultCode::NoTrust => Self::NoTrust,
                PathPaymentStrictReceiveResultCode::NotAuthorized => Self::NotAuthorized,
                PathPaymentStrictReceiveResultCode::LineFull => Self::LineFull,
                PathPaymentStrictReceiveResultCode::NoIssuer => Self::NoIssuer(Asset::read_xdr(r)?),
                PathPaymentStrictReceiveResultCode::TooFewOffers => Self::TooFewOffers,
                PathPaymentStrictReceiveResultCode::OfferCrossSelf => Self::OfferCrossSelf,
                PathPaymentStrictReceiveResultCode::OverSendmax => Self::OverSendmax,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for PathPaymentStrictReceiveResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success(v) => v.write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::Underfunded => ().write_xdr(w)?,
                Self::SrcNoTrust => ().write_xdr(w)?,
                Self::SrcNotAuthorized => ().write_xdr(w)?,
                Self::NoDestination => ().write_xdr(w)?,
                Self::NoTrust => ().write_xdr(w)?,
                Self::NotAuthorized => ().write_xdr(w)?,
                Self::LineFull => ().write_xdr(w)?,
                Self::NoIssuer(v) => v.write_xdr(w)?,
                Self::TooFewOffers => ().write_xdr(w)?,
                Self::OfferCrossSelf => ().write_xdr(w)?,
                Self::OverSendmax => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`PathPaymentStrictReceiveResult`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyPathPaymentStrictReceiveResult(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyPathPaymentStrictReceiveResult {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyPathPaymentStrictReceiveResult {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_success().cmp(&other.as_success()),
            -9 => self.as_no_issuer().cmp(&other.as_no_issuer()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyPathPaymentStrictReceiveResult {
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
                    <LazyPathPaymentStrictReceiveResultSuccess as LazyXdr>::xdr_validate(
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
                let field_len = <LazyAsset as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
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
                pos += <LazyPathPaymentStrictReceiveResultSuccess as LazyXdr>::xdr_len(
                    &buf[pos as usize..],
                );
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
                pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyPathPaymentStrictReceiveResult {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyPathPaymentStrictReceiveResult {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyPathPaymentStrictReceiveResult {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyPathPaymentStrictReceiveResult {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> PathPaymentStrictReceiveResultCode {
        // Validated — unwrap is safe.
        PathPaymentStrictReceiveResultCode::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `Success`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_success(&self) -> Option<LazyPathPaymentStrictReceiveResultSuccess> {
        if self.discriminant_i32() == 0 {
            Some(<LazyPathPaymentStrictReceiveResultSuccess as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `NoIssuer`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_no_issuer(&self) -> Option<LazyAsset> {
        if self.discriminant_i32() == -9 {
            Some(<LazyAsset as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&PathPaymentStrictReceiveResult> for LazyPathPaymentStrictReceiveResult {
    type Error = Error;
    fn try_from(val: &PathPaymentStrictReceiveResult) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyPathPaymentStrictReceiveResult> for PathPaymentStrictReceiveResult {
    type Error = Error;
    fn try_from(lazy: &LazyPathPaymentStrictReceiveResult) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
