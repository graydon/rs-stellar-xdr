#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// LedgerCloseMetaV2 is an XDR Struct defined as:
///
/// ```text
/// struct LedgerCloseMetaV2
/// {
///     LedgerCloseMetaExt ext;
///
///     LedgerHeaderHistoryEntry ledgerHeader;
///
///     GeneralizedTransactionSet txSet;
///
///     // NB: transactions are sorted in apply order here
///     // fees for all transactions are processed first
///     // followed by applying transactions
///     TransactionResultMetaV1 txProcessing<>;
///
///     // upgrades are applied last
///     UpgradeEntryMeta upgradesProcessing<>;
///
///     // other misc information attached to the ledger close
///     SCPHistoryEntry scpInfo<>;
///
///     // Size in bytes of live Soroban state, to support downstream
///     // systems calculating storage fees correctly.
///     uint64 totalByteSizeOfLiveSorobanState;
///
///     // TTL and data/code keys that have been evicted at this ledger.
///     LedgerKey evictedKeys<>;
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
pub struct LedgerCloseMetaV2 {
    pub ext: LedgerCloseMetaExt,
    pub ledger_header: LedgerHeaderHistoryEntry,
    pub tx_set: GeneralizedTransactionSet,
    pub tx_processing: VecM<TransactionResultMetaV1>,
    pub upgrades_processing: VecM<UpgradeEntryMeta>,
    pub scp_info: VecM<ScpHistoryEntry>,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub total_byte_size_of_live_soroban_state: u64,
    pub evicted_keys: VecM<LedgerKey>,
}

impl ReadXdr for LedgerCloseMetaV2 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: LedgerCloseMetaExt::read_xdr(r)?,
                ledger_header: LedgerHeaderHistoryEntry::read_xdr(r)?,
                tx_set: GeneralizedTransactionSet::read_xdr(r)?,
                tx_processing: VecM::<TransactionResultMetaV1>::read_xdr(r)?,
                upgrades_processing: VecM::<UpgradeEntryMeta>::read_xdr(r)?,
                scp_info: VecM::<ScpHistoryEntry>::read_xdr(r)?,
                total_byte_size_of_live_soroban_state: u64::read_xdr(r)?,
                evicted_keys: VecM::<LedgerKey>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerCloseMetaV2 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.ledger_header.write_xdr(w)?;
            self.tx_set.write_xdr(w)?;
            self.tx_processing.write_xdr(w)?;
            self.upgrades_processing.write_xdr(w)?;
            self.scp_info.write_xdr(w)?;
            self.total_byte_size_of_live_soroban_state.write_xdr(w)?;
            self.evicted_keys.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`LedgerCloseMetaV2`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyLedgerCloseMetaV2(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyLedgerCloseMetaV2 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyLedgerCloseMetaV2 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.ext().cmp(&other.ext()))
            .then_with(|| self.ledger_header().cmp(&other.ledger_header()))
            .then_with(|| self.tx_set().cmp(&other.tx_set()))
            .then_with(|| self.tx_processing().cmp(&other.tx_processing()))
            .then_with(|| self.upgrades_processing().cmp(&other.upgrades_processing()))
            .then_with(|| self.scp_info().cmp(&other.scp_info()))
            .then_with(|| {
                self.total_byte_size_of_live_soroban_state()
                    .cmp(&other.total_byte_size_of_live_soroban_state())
            })
            .then_with(|| self.evicted_keys().cmp(&other.evicted_keys()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyLedgerCloseMetaV2 {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyLedgerCloseMetaExt as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyLedgerHeaderHistoryEntry as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyGeneralizedTransactionSet as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyVecM<LazyTransactionResultMetaV1> as LazyXdr>::xdr_validate(
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
        let next_pos = pos.checked_add(8).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        {
            let field_len =
                <LazyVecM<LazyLedgerKey> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyLedgerCloseMetaExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerHeaderHistoryEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyGeneralizedTransactionSet as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyTransactionResultMetaV1> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyUpgradeEntryMeta> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyScpHistoryEntry> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        pos += <LazyVecM<LazyLedgerKey> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyLedgerCloseMetaV2 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyLedgerCloseMetaV2 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyLedgerCloseMetaV2 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyLedgerCloseMetaV2 {
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazyLedgerCloseMetaExt {
        <LazyLedgerCloseMetaExt as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `ledger_header`.
    #[must_use]
    pub fn ledger_header(&self) -> LazyLedgerHeaderHistoryEntry {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyLedgerCloseMetaExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyLedgerHeaderHistoryEntry as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `tx_set`.
    #[must_use]
    pub fn tx_set(&self) -> LazyGeneralizedTransactionSet {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyLedgerCloseMetaExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerHeaderHistoryEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyGeneralizedTransactionSet as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `tx_processing`.
    #[must_use]
    pub fn tx_processing(&self) -> LazyVecM<LazyTransactionResultMetaV1> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyLedgerCloseMetaExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerHeaderHistoryEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyGeneralizedTransactionSet as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyTransactionResultMetaV1> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `upgrades_processing`.
    #[must_use]
    pub fn upgrades_processing(&self) -> LazyVecM<LazyUpgradeEntryMeta> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyLedgerCloseMetaExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerHeaderHistoryEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyGeneralizedTransactionSet as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyTransactionResultMetaV1> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyUpgradeEntryMeta> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `scp_info`.
    #[must_use]
    pub fn scp_info(&self) -> LazyVecM<LazyScpHistoryEntry> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyLedgerCloseMetaExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerHeaderHistoryEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyGeneralizedTransactionSet as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyTransactionResultMetaV1> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyUpgradeEntryMeta> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyScpHistoryEntry> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `total_byte_size_of_live_soroban_state`.
    #[must_use]
    pub fn total_byte_size_of_live_soroban_state(&self) -> u64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyLedgerCloseMetaExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerHeaderHistoryEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyGeneralizedTransactionSet as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyTransactionResultMetaV1> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyUpgradeEntryMeta> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyScpHistoryEntry> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <u64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `evicted_keys`.
    #[must_use]
    pub fn evicted_keys(&self) -> LazyVecM<LazyLedgerKey> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyLedgerCloseMetaExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerHeaderHistoryEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyGeneralizedTransactionSet as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyTransactionResultMetaV1> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyUpgradeEntryMeta> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyScpHistoryEntry> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        <LazyVecM<LazyLedgerKey> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LedgerCloseMetaV2> for LazyLedgerCloseMetaV2 {
    type Error = Error;
    fn try_from(val: &LedgerCloseMetaV2) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyLedgerCloseMetaV2> for LedgerCloseMetaV2 {
    type Error = Error;
    fn try_from(lazy: &LazyLedgerCloseMetaV2) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
