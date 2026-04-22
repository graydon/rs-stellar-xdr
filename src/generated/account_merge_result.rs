#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// AccountMergeResult is an XDR Union defined as:
///
/// ```text
/// union AccountMergeResult switch (AccountMergeResultCode code)
/// {
/// case ACCOUNT_MERGE_SUCCESS:
///     int64 sourceAccountBalance; // how much got transferred from source account
/// case ACCOUNT_MERGE_MALFORMED:
/// case ACCOUNT_MERGE_NO_ACCOUNT:
/// case ACCOUNT_MERGE_IMMUTABLE_SET:
/// case ACCOUNT_MERGE_HAS_SUB_ENTRIES:
/// case ACCOUNT_MERGE_SEQNUM_TOO_FAR:
/// case ACCOUNT_MERGE_DEST_FULL:
/// case ACCOUNT_MERGE_IS_SPONSOR:
///     void;
/// };
/// ```
///
// union with discriminant AccountMergeResultCode
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
pub enum AccountMergeResult {
    Success(
        #[cfg_attr(
            all(feature = "serde", feature = "alloc"),
            serde_as(as = "NumberOrString")
        )]
        i64,
    ),
    Malformed,
    NoAccount,
    ImmutableSet,
    HasSubEntries,
    SeqnumTooFar,
    DestFull,
    IsSponsor,
}

#[cfg(feature = "alloc")]
impl Default for AccountMergeResult {
    fn default() -> Self {
        Self::Success(i64::default())
    }
}

impl AccountMergeResult {
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
            Self::Success(_) => "Success",
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
    pub const fn discriminant(&self) -> AccountMergeResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => AccountMergeResultCode::Success,
            Self::Malformed => AccountMergeResultCode::Malformed,
            Self::NoAccount => AccountMergeResultCode::NoAccount,
            Self::ImmutableSet => AccountMergeResultCode::ImmutableSet,
            Self::HasSubEntries => AccountMergeResultCode::HasSubEntries,
            Self::SeqnumTooFar => AccountMergeResultCode::SeqnumTooFar,
            Self::DestFull => AccountMergeResultCode::DestFull,
            Self::IsSponsor => AccountMergeResultCode::IsSponsor,
        }
    }

    #[must_use]
    pub const fn variants() -> [AccountMergeResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for AccountMergeResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<AccountMergeResultCode> for AccountMergeResult {
    #[must_use]
    fn discriminant(&self) -> AccountMergeResultCode {
        Self::discriminant(self)
    }
}

impl Variants<AccountMergeResultCode> for AccountMergeResult {
    fn variants() -> slice::Iter<'static, AccountMergeResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<AccountMergeResultCode> for AccountMergeResult {}

impl ReadXdr for AccountMergeResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: AccountMergeResultCode = <AccountMergeResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                AccountMergeResultCode::Success => Self::Success(i64::read_xdr(r)?),
                AccountMergeResultCode::Malformed => Self::Malformed,
                AccountMergeResultCode::NoAccount => Self::NoAccount,
                AccountMergeResultCode::ImmutableSet => Self::ImmutableSet,
                AccountMergeResultCode::HasSubEntries => Self::HasSubEntries,
                AccountMergeResultCode::SeqnumTooFar => Self::SeqnumTooFar,
                AccountMergeResultCode::DestFull => Self::DestFull,
                AccountMergeResultCode::IsSponsor => Self::IsSponsor,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for AccountMergeResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success(v) => v.write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::NoAccount => ().write_xdr(w)?,
                Self::ImmutableSet => ().write_xdr(w)?,
                Self::HasSubEntries => ().write_xdr(w)?,
                Self::SeqnumTooFar => ().write_xdr(w)?,
                Self::DestFull => ().write_xdr(w)?,
                Self::IsSponsor => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`AccountMergeResult`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyAccountMergeResult(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyAccountMergeResult {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyAccountMergeResult {
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
impl LazyXdr for LazyAccountMergeResult {
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
                let field_len = <i64 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
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
                pos += <i64 as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyAccountMergeResult {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyAccountMergeResult {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyAccountMergeResult {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyAccountMergeResult {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> AccountMergeResultCode {
        // Validated — unwrap is safe.
        AccountMergeResultCode::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `Success`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_success(&self) -> Option<i64> {
        if self.discriminant_i32() == 0 {
            Some(<i64 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&AccountMergeResult> for LazyAccountMergeResult {
    type Error = Error;
    fn try_from(val: &AccountMergeResult) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyAccountMergeResult> for AccountMergeResult {
    type Error = Error;
    fn try_from(lazy: &LazyAccountMergeResult) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
