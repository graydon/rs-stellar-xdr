#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// TrustLineEntry is an XDR Struct defined as:
///
/// ```text
/// struct TrustLineEntry
/// {
///     AccountID accountID;  // account this trustline belongs to
///     TrustLineAsset asset; // type of asset (with issuer)
///     int64 balance;        // how much of this asset the user has.
///                           // Asset defines the unit for this;
///
///     int64 limit;  // balance cannot be above this
///     uint32 flags; // see TrustLineFlags
///
///     // reserved for future use
///     union switch (int v)
///     {
///     case 0:
///         void;
///     case 1:
///         struct
///         {
///             Liabilities liabilities;
///
///             union switch (int v)
///             {
///             case 0:
///                 void;
///             case 2:
///                 TrustLineEntryExtensionV2 v2;
///             }
///             ext;
///         } v1;
///     }
///     ext;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct TrustLineEntry {
    pub account_id: AccountId,
    pub asset: TrustLineAsset,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub balance: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub limit: i64,
    pub flags: u32,
    pub ext: TrustLineEntryExt,
}

impl ReadXdr for TrustLineEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                account_id: AccountId::read_xdr(r)?,
                asset: TrustLineAsset::read_xdr(r)?,
                balance: i64::read_xdr(r)?,
                limit: i64::read_xdr(r)?,
                flags: u32::read_xdr(r)?,
                ext: TrustLineEntryExt::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TrustLineEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.account_id.write_xdr(w)?;
            self.asset.write_xdr(w)?;
            self.balance.write_xdr(w)?;
            self.limit.write_xdr(w)?;
            self.flags.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`TrustLineEntry`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyTrustLineEntry(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyTrustLineEntry {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyTrustLineEntry {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.account_id().cmp(&other.account_id()))
            .then_with(|| self.asset().cmp(&other.asset()))
            .then_with(|| self.balance().cmp(&other.balance()))
            .then_with(|| self.limit().cmp(&other.limit()))
            .then_with(|| self.flags().cmp(&other.flags()))
            .then_with(|| self.ext().cmp(&other.ext()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyTrustLineEntry {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazyAccountId as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyTrustLineAsset as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(20).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        {
            let field_len =
                <LazyTrustLineEntryExt as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyTrustLineAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 20;
        pos += <LazyTrustLineEntryExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyTrustLineEntry {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyTrustLineEntry {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyTrustLineEntry {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyTrustLineEntry {
    /// Access field `account_id`.
    #[must_use]
    pub fn account_id(&self) -> LazyAccountId {
        <LazyAccountId as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `asset`.
    #[must_use]
    pub fn asset(&self) -> LazyTrustLineAsset {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyTrustLineAsset as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `balance`.
    #[must_use]
    pub fn balance(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyTrustLineAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `limit`.
    #[must_use]
    pub fn limit(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyTrustLineAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `flags`.
    #[must_use]
    pub fn flags(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyTrustLineAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 16;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazyTrustLineEntryExt {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyTrustLineAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 20;
        <LazyTrustLineEntryExt as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&TrustLineEntry> for LazyTrustLineEntry {
    type Error = Error;
    fn try_from(val: &TrustLineEntry) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyTrustLineEntry> for TrustLineEntry {
    type Error = Error;
    fn try_from(lazy: &LazyTrustLineEntry) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
