#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// LedgerCloseMetaV0 is an XDR Struct defined as:
///
/// ```text
/// struct LedgerCloseMetaV0
/// {
///     LedgerHeaderHistoryEntry ledgerHeader;
///     // NB: txSet is sorted in "Hash order"
///     TransactionSet txSet;
///
///     // NB: transactions are sorted in apply order here
///     // fees for all transactions are processed first
///     // followed by applying transactions
///     TransactionResultMeta txProcessing<>;
///
///     // upgrades are applied last
///     UpgradeEntryMeta upgradesProcessing<>;
///
///     // other misc information attached to the ledger close
///     SCPHistoryEntry scpInfo<>;
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
pub struct LedgerCloseMetaV0 {
    pub ledger_header: LedgerHeaderHistoryEntry,
    pub tx_set: TransactionSet,
    pub tx_processing: VecM<TransactionResultMeta>,
    pub upgrades_processing: VecM<UpgradeEntryMeta>,
    pub scp_info: VecM<ScpHistoryEntry>,
}

impl ReadXdr for LedgerCloseMetaV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ledger_header: LedgerHeaderHistoryEntry::read_xdr(r)?,
                tx_set: TransactionSet::read_xdr(r)?,
                tx_processing: VecM::<TransactionResultMeta>::read_xdr(r)?,
                upgrades_processing: VecM::<UpgradeEntryMeta>::read_xdr(r)?,
                scp_info: VecM::<ScpHistoryEntry>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerCloseMetaV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_header.write_xdr(w)?;
            self.tx_set.write_xdr(w)?;
            self.tx_processing.write_xdr(w)?;
            self.upgrades_processing.write_xdr(w)?;
            self.scp_info.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`LedgerCloseMetaV0`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyLedgerCloseMetaV0(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyLedgerCloseMetaV0 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyLedgerCloseMetaV0 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.ledger_header().cmp(&other.ledger_header()))
            .then_with(|| self.tx_set().cmp(&other.tx_set()))
            .then_with(|| self.tx_processing().cmp(&other.tx_processing()))
            .then_with(|| self.upgrades_processing().cmp(&other.upgrades_processing()))
            .then_with(|| self.scp_info().cmp(&other.scp_info()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyLedgerCloseMetaV0 {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazyLedgerHeaderHistoryEntry as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyTransactionSet as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyVecM<LazyTransactionResultMeta> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyVecM<LazyUpgradeEntryMeta> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyVecM<LazyScpHistoryEntry> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyLedgerHeaderHistoryEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyTransactionSet as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyTransactionResultMeta> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyUpgradeEntryMeta> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyScpHistoryEntry> as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyLedgerCloseMetaV0 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyLedgerCloseMetaV0 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyLedgerCloseMetaV0 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyLedgerCloseMetaV0 {
    /// Access field `ledger_header`.
    #[must_use]
    pub fn ledger_header(&self) -> LazyLedgerHeaderHistoryEntry {
        <LazyLedgerHeaderHistoryEntry as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `tx_set`.
    #[must_use]
    pub fn tx_set(&self) -> LazyTransactionSet {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyLedgerHeaderHistoryEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyTransactionSet as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `tx_processing`.
    #[must_use]
    pub fn tx_processing(&self) -> LazyVecM<LazyTransactionResultMeta> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyLedgerHeaderHistoryEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyTransactionSet as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyTransactionResultMeta> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `upgrades_processing`.
    #[must_use]
    pub fn upgrades_processing(&self) -> LazyVecM<LazyUpgradeEntryMeta> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyLedgerHeaderHistoryEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyTransactionSet as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyTransactionResultMeta> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyUpgradeEntryMeta> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `scp_info`.
    #[must_use]
    pub fn scp_info(&self) -> LazyVecM<LazyScpHistoryEntry> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyLedgerHeaderHistoryEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyTransactionSet as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyTransactionResultMeta> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyUpgradeEntryMeta> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyScpHistoryEntry> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LedgerCloseMetaV0> for LazyLedgerCloseMetaV0 {
    type Error = Error;
    fn try_from(val: &LedgerCloseMetaV0) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyLedgerCloseMetaV0> for LedgerCloseMetaV0 {
    type Error = Error;
    fn try_from(lazy: &LazyLedgerCloseMetaV0) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
