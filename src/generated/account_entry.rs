#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// AccountEntry is an XDR Struct defined as:
///
/// ```text
/// struct AccountEntry
/// {
///     AccountID accountID;      // master public key for this account
///     int64 balance;            // in stroops
///     SequenceNumber seqNum;    // last sequence number used for this account
///     uint32 numSubEntries;     // number of sub-entries this account has
///                               // drives the reserve
///     AccountID* inflationDest; // Account to vote for during inflation
///     uint32 flags;             // see AccountFlags
///
///     string32 homeDomain; // can be used for reverse federation and memo lookup
///
///     // fields used for signatures
///     // thresholds stores unsigned bytes: [weight of master|low|medium|high]
///     Thresholds thresholds;
///
///     Signer signers<MAX_SIGNERS>; // possible signers for this account
///
///     // reserved for future use
///     union switch (int v)
///     {
///     case 0:
///         void;
///     case 1:
///         AccountEntryExtensionV1 v1;
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
pub struct AccountEntry {
    pub account_id: AccountId,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub balance: i64,
    pub seq_num: SequenceNumber,
    pub num_sub_entries: u32,
    pub inflation_dest: Option<AccountId>,
    pub flags: u32,
    pub home_domain: String32,
    pub thresholds: Thresholds,
    pub signers: VecM<Signer, 20>,
    pub ext: AccountEntryExt,
}

impl ReadXdr for AccountEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                account_id: AccountId::read_xdr(r)?,
                balance: i64::read_xdr(r)?,
                seq_num: SequenceNumber::read_xdr(r)?,
                num_sub_entries: u32::read_xdr(r)?,
                inflation_dest: Option::<AccountId>::read_xdr(r)?,
                flags: u32::read_xdr(r)?,
                home_domain: String32::read_xdr(r)?,
                thresholds: Thresholds::read_xdr(r)?,
                signers: VecM::<Signer, 20>::read_xdr(r)?,
                ext: AccountEntryExt::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for AccountEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.account_id.write_xdr(w)?;
            self.balance.write_xdr(w)?;
            self.seq_num.write_xdr(w)?;
            self.num_sub_entries.write_xdr(w)?;
            self.inflation_dest.write_xdr(w)?;
            self.flags.write_xdr(w)?;
            self.home_domain.write_xdr(w)?;
            self.thresholds.write_xdr(w)?;
            self.signers.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`AccountEntry`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyAccountEntry(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyAccountEntry {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyAccountEntry {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.account_id().cmp(&other.account_id()))
            .then_with(|| self.balance().cmp(&other.balance()))
            .then_with(|| self.seq_num().cmp(&other.seq_num()))
            .then_with(|| self.num_sub_entries().cmp(&other.num_sub_entries()))
            .then_with(|| self.inflation_dest().cmp(&other.inflation_dest()))
            .then_with(|| self.flags().cmp(&other.flags()))
            .then_with(|| self.home_domain().cmp(&other.home_domain()))
            .then_with(|| self.thresholds().cmp(&other.thresholds()))
            .then_with(|| self.signers().cmp(&other.signers()))
            .then_with(|| self.ext().cmp(&other.ext()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyAccountEntry {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazyAccountId as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(20).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        {
            let field_len =
                <LazyOption<LazyAccountId> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(4).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        {
            let field_len = <LazyString32 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(4).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyThresholds as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        {
            let field_len =
                <LazyVecM<LazySigner, 20> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyAccountEntryExt as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 20;
        pos += <LazyOption<LazyAccountId> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        pos += <LazyString32 as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        pos += <LazyVecM<LazySigner, 20> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyAccountEntryExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyAccountEntry {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyAccountEntry {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyAccountEntry {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyAccountEntry {
    /// Access field `account_id`.
    #[must_use]
    pub fn account_id(&self) -> LazyAccountId {
        <LazyAccountId as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `balance`.
    #[must_use]
    pub fn balance(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `seq_num`.
    #[must_use]
    pub fn seq_num(&self) -> LazySequenceNumber {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        <LazySequenceNumber as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `num_sub_entries`.
    #[must_use]
    pub fn num_sub_entries(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 16;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `inflation_dest`.
    #[must_use]
    pub fn inflation_dest(&self) -> LazyOption<LazyAccountId> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 20;
        <LazyOption<LazyAccountId> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `flags`.
    #[must_use]
    pub fn flags(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 20;
        pos += <LazyOption<LazyAccountId> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `home_domain`.
    #[must_use]
    pub fn home_domain(&self) -> LazyString32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 20;
        pos += <LazyOption<LazyAccountId> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        <LazyString32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `thresholds`.
    #[must_use]
    pub fn thresholds(&self) -> LazyThresholds {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 20;
        pos += <LazyOption<LazyAccountId> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        pos += <LazyString32 as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyThresholds as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `signers`.
    #[must_use]
    pub fn signers(&self) -> LazyVecM<LazySigner, 20> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 20;
        pos += <LazyOption<LazyAccountId> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        pos += <LazyString32 as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        <LazyVecM<LazySigner, 20> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazyAccountEntryExt {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 20;
        pos += <LazyOption<LazyAccountId> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        pos += <LazyString32 as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        pos += <LazyVecM<LazySigner, 20> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyAccountEntryExt as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&AccountEntry> for LazyAccountEntry {
    type Error = Error;
    fn try_from(val: &AccountEntry) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyAccountEntry> for AccountEntry {
    type Error = Error;
    fn try_from(lazy: &LazyAccountEntry) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
