#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// AccountMergeResultCode is an XDR Enum defined as:
///
/// ```text
/// enum AccountMergeResultCode
/// {
///     // codes considered as "success" for the operation
///     ACCOUNT_MERGE_SUCCESS = 0,
///     // codes considered as "failure" for the operation
///     ACCOUNT_MERGE_MALFORMED = -1,       // can't merge onto itself
///     ACCOUNT_MERGE_NO_ACCOUNT = -2,      // destination does not exist
///     ACCOUNT_MERGE_IMMUTABLE_SET = -3,   // source account has AUTH_IMMUTABLE set
///     ACCOUNT_MERGE_HAS_SUB_ENTRIES = -4, // account has trust lines/offers
///     ACCOUNT_MERGE_SEQNUM_TOO_FAR = -5,  // sequence number is over max allowed
///     ACCOUNT_MERGE_DEST_FULL = -6,       // can't add source balance to
///                                         // destination balance
///     ACCOUNT_MERGE_IS_SPONSOR = -7       // can't merge account that is a sponsor
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
pub enum AccountMergeResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    Malformed = -1,
    NoAccount = -2,
    ImmutableSet = -3,
    HasSubEntries = -4,
    SeqnumTooFar = -5,
    DestFull = -6,
    IsSponsor = -7,
}

impl AccountMergeResultCode {
    const _VARIANTS: &[AccountMergeResultCode] = &[
        AccountMergeResultCode::Success,
        AccountMergeResultCode::Malformed,
        AccountMergeResultCode::NoAccount,
        AccountMergeResultCode::ImmutableSet,
        AccountMergeResultCode::HasSubEntries,
        AccountMergeResultCode::SeqnumTooFar,
        AccountMergeResultCode::DestFull,
        AccountMergeResultCode::IsSponsor,
    ];
    pub const VARIANTS: [AccountMergeResultCode; Self::_VARIANTS.len()] = {
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
        "NoAccount",
        "ImmutableSet",
        "HasSubEntries",
        "SeqnumTooFar",
        "DestFull",
        "IsSponsor",
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
            Self::NoAccount => "NoAccount",
            Self::ImmutableSet => "ImmutableSet",
            Self::HasSubEntries => "HasSubEntries",
            Self::SeqnumTooFar => "SeqnumTooFar",
            Self::DestFull => "DestFull",
            Self::IsSponsor => "IsSponsor",
        }
    }

    #[must_use]
    pub const fn variants() -> [AccountMergeResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for AccountMergeResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<AccountMergeResultCode> for AccountMergeResultCode {
    fn variants() -> slice::Iter<'static, AccountMergeResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for AccountMergeResultCode {}

impl fmt::Display for AccountMergeResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for AccountMergeResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => AccountMergeResultCode::Success,
            -1 => AccountMergeResultCode::Malformed,
            -2 => AccountMergeResultCode::NoAccount,
            -3 => AccountMergeResultCode::ImmutableSet,
            -4 => AccountMergeResultCode::HasSubEntries,
            -5 => AccountMergeResultCode::SeqnumTooFar,
            -6 => AccountMergeResultCode::DestFull,
            -7 => AccountMergeResultCode::IsSponsor,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<AccountMergeResultCode> for i32 {
    #[must_use]
    fn from(e: AccountMergeResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for AccountMergeResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for AccountMergeResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}

#[cfg(feature = "alloc")]
// Enum AccountMergeResultCode: scalar lazy type — impl LazyXdr directly on the enum.
impl LazyXdr for AccountMergeResultCode {
    const FIXED_XDR_SIZE: Option<u32> = Some(4);

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let v = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let _ = AccountMergeResultCode::try_from(v)?;
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
        AccountMergeResultCode::try_from(v).unwrap()
    }
}
