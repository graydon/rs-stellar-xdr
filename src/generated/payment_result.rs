#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// PaymentResult is an XDR Union defined as:
///
/// ```text
/// union PaymentResult switch (PaymentResultCode code)
/// {
/// case PAYMENT_SUCCESS:
///     void;
/// case PAYMENT_MALFORMED:
/// case PAYMENT_UNDERFUNDED:
/// case PAYMENT_SRC_NO_TRUST:
/// case PAYMENT_SRC_NOT_AUTHORIZED:
/// case PAYMENT_NO_DESTINATION:
/// case PAYMENT_NO_TRUST:
/// case PAYMENT_NOT_AUTHORIZED:
/// case PAYMENT_LINE_FULL:
/// case PAYMENT_NO_ISSUER:
///     void;
/// };
/// ```
///
// union with discriminant PaymentResultCode
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
pub enum PaymentResult {
    Success,
    Malformed,
    Underfunded,
    SrcNoTrust,
    SrcNotAuthorized,
    NoDestination,
    NoTrust,
    NotAuthorized,
    LineFull,
    NoIssuer,
}

#[cfg(feature = "alloc")]
impl Default for PaymentResult {
    fn default() -> Self {
        Self::Success
    }
}

impl PaymentResult {
    const _VARIANTS: &[PaymentResultCode] = &[
        PaymentResultCode::Success,
        PaymentResultCode::Malformed,
        PaymentResultCode::Underfunded,
        PaymentResultCode::SrcNoTrust,
        PaymentResultCode::SrcNotAuthorized,
        PaymentResultCode::NoDestination,
        PaymentResultCode::NoTrust,
        PaymentResultCode::NotAuthorized,
        PaymentResultCode::LineFull,
        PaymentResultCode::NoIssuer,
    ];
    pub const VARIANTS: [PaymentResultCode; Self::_VARIANTS.len()] = {
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
            Self::Underfunded => "Underfunded",
            Self::SrcNoTrust => "SrcNoTrust",
            Self::SrcNotAuthorized => "SrcNotAuthorized",
            Self::NoDestination => "NoDestination",
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
            Self::LineFull => "LineFull",
            Self::NoIssuer => "NoIssuer",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> PaymentResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => PaymentResultCode::Success,
            Self::Malformed => PaymentResultCode::Malformed,
            Self::Underfunded => PaymentResultCode::Underfunded,
            Self::SrcNoTrust => PaymentResultCode::SrcNoTrust,
            Self::SrcNotAuthorized => PaymentResultCode::SrcNotAuthorized,
            Self::NoDestination => PaymentResultCode::NoDestination,
            Self::NoTrust => PaymentResultCode::NoTrust,
            Self::NotAuthorized => PaymentResultCode::NotAuthorized,
            Self::LineFull => PaymentResultCode::LineFull,
            Self::NoIssuer => PaymentResultCode::NoIssuer,
        }
    }

    #[must_use]
    pub const fn variants() -> [PaymentResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for PaymentResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<PaymentResultCode> for PaymentResult {
    #[must_use]
    fn discriminant(&self) -> PaymentResultCode {
        Self::discriminant(self)
    }
}

impl Variants<PaymentResultCode> for PaymentResult {
    fn variants() -> slice::Iter<'static, PaymentResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<PaymentResultCode> for PaymentResult {}

impl ReadXdr for PaymentResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: PaymentResultCode = <PaymentResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                PaymentResultCode::Success => Self::Success,
                PaymentResultCode::Malformed => Self::Malformed,
                PaymentResultCode::Underfunded => Self::Underfunded,
                PaymentResultCode::SrcNoTrust => Self::SrcNoTrust,
                PaymentResultCode::SrcNotAuthorized => Self::SrcNotAuthorized,
                PaymentResultCode::NoDestination => Self::NoDestination,
                PaymentResultCode::NoTrust => Self::NoTrust,
                PaymentResultCode::NotAuthorized => Self::NotAuthorized,
                PaymentResultCode::LineFull => Self::LineFull,
                PaymentResultCode::NoIssuer => Self::NoIssuer,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for PaymentResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::Underfunded => ().write_xdr(w)?,
                Self::SrcNoTrust => ().write_xdr(w)?,
                Self::SrcNotAuthorized => ().write_xdr(w)?,
                Self::NoDestination => ().write_xdr(w)?,
                Self::NoTrust => ().write_xdr(w)?,
                Self::NotAuthorized => ().write_xdr(w)?,
                Self::LineFull => ().write_xdr(w)?,
                Self::NoIssuer => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`PaymentResult`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyPaymentResult(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyPaymentResult {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyPaymentResult {
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
impl LazyXdr for LazyPaymentResult {
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
            -9 => {
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
            -9 => {
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
impl From<LazyHandle> for LazyPaymentResult {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyPaymentResult {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyPaymentResult {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyPaymentResult {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> PaymentResultCode {
        // Validated — unwrap is safe.
        PaymentResultCode::try_from(self.discriminant_i32()).unwrap()
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&PaymentResult> for LazyPaymentResult {
    type Error = Error;
    fn try_from(val: &PaymentResult) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyPaymentResult> for PaymentResult {
    type Error = Error;
    fn try_from(lazy: &LazyPaymentResult) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
