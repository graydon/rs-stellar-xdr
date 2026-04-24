#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ClawbackClaimableBalanceResultCode is an XDR Enum defined as:
///
/// ```text
/// enum ClawbackClaimableBalanceResultCode
/// {
///     // codes considered as "success" for the operation
///     CLAWBACK_CLAIMABLE_BALANCE_SUCCESS = 0,
///
///     // codes considered as "failure" for the operation
///     CLAWBACK_CLAIMABLE_BALANCE_DOES_NOT_EXIST = -1,
///     CLAWBACK_CLAIMABLE_BALANCE_NOT_ISSUER = -2,
///     CLAWBACK_CLAIMABLE_BALANCE_NOT_CLAWBACK_ENABLED = -3
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
pub enum ClawbackClaimableBalanceResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    DoesNotExist = -1,
    NotIssuer = -2,
    NotClawbackEnabled = -3,
}

impl ClawbackClaimableBalanceResultCode {
    const _VARIANTS: &[ClawbackClaimableBalanceResultCode] = &[
        ClawbackClaimableBalanceResultCode::Success,
        ClawbackClaimableBalanceResultCode::DoesNotExist,
        ClawbackClaimableBalanceResultCode::NotIssuer,
        ClawbackClaimableBalanceResultCode::NotClawbackEnabled,
    ];
    pub const VARIANTS: [ClawbackClaimableBalanceResultCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Success", "DoesNotExist", "NotIssuer", "NotClawbackEnabled"];
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
            Self::NotIssuer => "NotIssuer",
            Self::NotClawbackEnabled => "NotClawbackEnabled",
        }
    }

    #[must_use]
    pub const fn variants() -> [ClawbackClaimableBalanceResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ClawbackClaimableBalanceResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ClawbackClaimableBalanceResultCode> for ClawbackClaimableBalanceResultCode {
    fn variants() -> slice::Iter<'static, ClawbackClaimableBalanceResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ClawbackClaimableBalanceResultCode {}

impl fmt::Display for ClawbackClaimableBalanceResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ClawbackClaimableBalanceResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ClawbackClaimableBalanceResultCode::Success,
            -1 => ClawbackClaimableBalanceResultCode::DoesNotExist,
            -2 => ClawbackClaimableBalanceResultCode::NotIssuer,
            -3 => ClawbackClaimableBalanceResultCode::NotClawbackEnabled,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ClawbackClaimableBalanceResultCode> for i32 {
    #[must_use]
    fn from(e: ClawbackClaimableBalanceResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for ClawbackClaimableBalanceResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ClawbackClaimableBalanceResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}

#[cfg(feature = "alloc")]
// Enum ClawbackClaimableBalanceResultCode: scalar lazy type — impl LazyXdr directly on the enum.
impl LazyXdr for ClawbackClaimableBalanceResultCode {
    const FIXED_XDR_SIZE: Option<u32> = Some(4);

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let bytes: [u8; 4] = buf[..4].try_into().unwrap();
        let v = i32::from_be_bytes(bytes);
        let _ = ClawbackClaimableBalanceResultCode::try_from(v)?;
        Ok(4)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        4
    }

    #[inline]
    fn from_xdr_consume(_parent: &LazyHandle, buf: &mut &[u8]) -> Self {
        let bytes: [u8; 4] = buf[..4].try_into().unwrap();
        *buf = &buf[4..];
        let v = i32::from_be_bytes(bytes);
        // SAFETY: data was validated; unwrap is infallible.
        ClawbackClaimableBalanceResultCode::try_from(v).unwrap()
    }
}
