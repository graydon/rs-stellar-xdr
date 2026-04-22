#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// AllowTrustResultCode is an XDR Enum defined as:
///
/// ```text
/// enum AllowTrustResultCode
/// {
///     // codes considered as "success" for the operation
///     ALLOW_TRUST_SUCCESS = 0,
///     // codes considered as "failure" for the operation
///     ALLOW_TRUST_MALFORMED = -1,     // asset is not ASSET_TYPE_ALPHANUM
///     ALLOW_TRUST_NO_TRUST_LINE = -2, // trustor does not have a trustline
///                                     // source account does not require trust
///     ALLOW_TRUST_TRUST_NOT_REQUIRED = -3,
///     ALLOW_TRUST_CANT_REVOKE = -4,      // source account can't revoke trust,
///     ALLOW_TRUST_SELF_NOT_ALLOWED = -5, // trusting self is not allowed
///     ALLOW_TRUST_LOW_RESERVE = -6       // claimable balances can't be created
///                                        // on revoke due to low reserves
/// };
/// ```
///
// enum
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[repr(i32)]
pub enum AllowTrustResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    Malformed = -1,
    NoTrustLine = -2,
    TrustNotRequired = -3,
    CantRevoke = -4,
    SelfNotAllowed = -5,
    LowReserve = -6,
}

impl AllowTrustResultCode {
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
    pub const fn variants() -> [AllowTrustResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for AllowTrustResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<AllowTrustResultCode> for AllowTrustResultCode {
    fn variants() -> slice::Iter<'static, AllowTrustResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for AllowTrustResultCode {}

impl fmt::Display for AllowTrustResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for AllowTrustResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => AllowTrustResultCode::Success,
            -1 => AllowTrustResultCode::Malformed,
            -2 => AllowTrustResultCode::NoTrustLine,
            -3 => AllowTrustResultCode::TrustNotRequired,
            -4 => AllowTrustResultCode::CantRevoke,
            -5 => AllowTrustResultCode::SelfNotAllowed,
            -6 => AllowTrustResultCode::LowReserve,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<AllowTrustResultCode> for i32 {
    #[must_use]
    fn from(e: AllowTrustResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for AllowTrustResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for AllowTrustResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}

#[cfg(feature = "alloc")]
// Enum AllowTrustResultCode: scalar lazy type — impl LazyXdr directly on the enum.
impl LazyXdr for AllowTrustResultCode {
    const FIXED_XDR_SIZE: Option<u32> = Some(4);

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let v = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let _ = AllowTrustResultCode::try_from(v)?;
        Ok(4)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        4
    }

    #[inline]
    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let b = &parent.as_slice()[offset as usize..];
        let v = i32::from_be_bytes([b[0], b[1], b[2], b[3]]);
        // SAFETY: data was validated; unwrap is infallible.
        AllowTrustResultCode::try_from(v).unwrap()
    }
}
