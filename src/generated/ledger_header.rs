#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// LedgerHeader is an XDR Struct defined as:
///
/// ```text
/// struct LedgerHeader
/// {
///     uint32 ledgerVersion;    // the protocol version of the ledger
///     Hash previousLedgerHash; // hash of the previous ledger header
///     StellarValue scpValue;   // what consensus agreed to
///     Hash txSetResultHash;    // the TransactionResultSet that led to this ledger
///     Hash bucketListHash;     // hash of the ledger state
///
///     uint32 ledgerSeq; // sequence number of this ledger
///
///     int64 totalCoins; // total number of stroops in existence.
///                       // 10,000,000 stroops in 1 XLM
///
///     int64 feePool;       // fees burned since last inflation run
///     uint32 inflationSeq; // inflation sequence number
///
///     uint64 idPool; // last used global ID, used for generating objects
///
///     uint32 baseFee;     // base fee per operation in stroops
///     uint32 baseReserve; // account base reserve in stroops
///
///     uint32 maxTxSetSize; // maximum size a transaction set can be
///
///     Hash skipList[4]; // hashes of ledgers in the past. allows you to jump back
///                       // in time without walking the chain back ledger by ledger
///                       // each slot contains the oldest ledger that is mod of
///                       // either 50  5000  50000 or 500000 depending on index
///                       // skipList[0] mod(50), skipList[1] mod(5000), etc
///
///     // reserved for future use
///     union switch (int v)
///     {
///     case 0:
///         void;
///     case 1:
///         LedgerHeaderExtensionV1 v1;
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
pub struct LedgerHeader {
    pub ledger_version: u32,
    pub previous_ledger_hash: Hash,
    pub scp_value: StellarValue,
    pub tx_set_result_hash: Hash,
    pub bucket_list_hash: Hash,
    pub ledger_seq: u32,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub total_coins: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub fee_pool: i64,
    pub inflation_seq: u32,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub id_pool: u64,
    pub base_fee: u32,
    pub base_reserve: u32,
    pub max_tx_set_size: u32,
    pub skip_list: [Hash; 4],
    pub ext: LedgerHeaderExt,
}

impl ReadXdr for LedgerHeader {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ledger_version: u32::read_xdr(r)?,
                previous_ledger_hash: Hash::read_xdr(r)?,
                scp_value: StellarValue::read_xdr(r)?,
                tx_set_result_hash: Hash::read_xdr(r)?,
                bucket_list_hash: Hash::read_xdr(r)?,
                ledger_seq: u32::read_xdr(r)?,
                total_coins: i64::read_xdr(r)?,
                fee_pool: i64::read_xdr(r)?,
                inflation_seq: u32::read_xdr(r)?,
                id_pool: u64::read_xdr(r)?,
                base_fee: u32::read_xdr(r)?,
                base_reserve: u32::read_xdr(r)?,
                max_tx_set_size: u32::read_xdr(r)?,
                skip_list: <[Hash; 4]>::read_xdr(r)?,
                ext: LedgerHeaderExt::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerHeader {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_version.write_xdr(w)?;
            self.previous_ledger_hash.write_xdr(w)?;
            self.scp_value.write_xdr(w)?;
            self.tx_set_result_hash.write_xdr(w)?;
            self.bucket_list_hash.write_xdr(w)?;
            self.ledger_seq.write_xdr(w)?;
            self.total_coins.write_xdr(w)?;
            self.fee_pool.write_xdr(w)?;
            self.inflation_seq.write_xdr(w)?;
            self.id_pool.write_xdr(w)?;
            self.base_fee.write_xdr(w)?;
            self.base_reserve.write_xdr(w)?;
            self.max_tx_set_size.write_xdr(w)?;
            self.skip_list.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`LedgerHeader`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyLedgerHeader(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyLedgerHeader {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyLedgerHeader {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.ledger_version().cmp(&other.ledger_version()))
            .then_with(|| {
                self.previous_ledger_hash()
                    .cmp(&other.previous_ledger_hash())
            })
            .then_with(|| self.scp_value().cmp(&other.scp_value()))
            .then_with(|| self.tx_set_result_hash().cmp(&other.tx_set_result_hash()))
            .then_with(|| self.bucket_list_hash().cmp(&other.bucket_list_hash()))
            .then_with(|| self.ledger_seq().cmp(&other.ledger_seq()))
            .then_with(|| self.total_coins().cmp(&other.total_coins()))
            .then_with(|| self.fee_pool().cmp(&other.fee_pool()))
            .then_with(|| self.inflation_seq().cmp(&other.inflation_seq()))
            .then_with(|| self.id_pool().cmp(&other.id_pool()))
            .then_with(|| self.base_fee().cmp(&other.base_fee()))
            .then_with(|| self.base_reserve().cmp(&other.base_reserve()))
            .then_with(|| self.max_tx_set_size().cmp(&other.max_tx_set_size()))
            .then_with(|| self.skip_list().cmp(&other.skip_list()))
            .then_with(|| self.ext().cmp(&other.ext()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyLedgerHeader {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(36).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyHash as LazyXdr>::xdr_validate(&buf[(pos + 4) as usize..], depth)?;
        pos = next_pos;
        {
            let field_len =
                <LazyStellarValue as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(236).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyHash as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        <LazyHash as LazyXdr>::xdr_validate(&buf[(pos + 32) as usize..], depth)?;
        <LazyFixedArray<LazyHash, 4> as LazyXdr>::xdr_validate(
            &buf[(pos + 108) as usize..],
            depth,
        )?;
        pos = next_pos;
        {
            let field_len =
                <LazyLedgerHeaderExt as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += 36;
        pos += <LazyStellarValue as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 236;
        pos += <LazyLedgerHeaderExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyLedgerHeader {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyLedgerHeader {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyLedgerHeader {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyLedgerHeader {
    /// Access field `ledger_version`.
    #[must_use]
    pub fn ledger_version(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `previous_ledger_hash`.
    #[must_use]
    pub fn previous_ledger_hash(&self) -> LazyHash {
        <LazyHash as LazyXdr>::from_xdr_at(&self.0, 4)
    }
    /// Access field `scp_value`.
    #[must_use]
    pub fn scp_value(&self) -> LazyStellarValue {
        <LazyStellarValue as LazyXdr>::from_xdr_at(&self.0, 36)
    }
    /// Access field `tx_set_result_hash`.
    #[must_use]
    pub fn tx_set_result_hash(&self) -> LazyHash {
        let buf = self.0.as_slice();
        let mut pos: u32 = 36;
        pos += <LazyStellarValue as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyHash as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `bucket_list_hash`.
    #[must_use]
    pub fn bucket_list_hash(&self) -> LazyHash {
        let buf = self.0.as_slice();
        let mut pos: u32 = 36;
        pos += <LazyStellarValue as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 32;
        <LazyHash as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `ledger_seq`.
    #[must_use]
    pub fn ledger_seq(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 36;
        pos += <LazyStellarValue as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 64;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `total_coins`.
    #[must_use]
    pub fn total_coins(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 36;
        pos += <LazyStellarValue as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 68;
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `fee_pool`.
    #[must_use]
    pub fn fee_pool(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 36;
        pos += <LazyStellarValue as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 76;
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `inflation_seq`.
    #[must_use]
    pub fn inflation_seq(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 36;
        pos += <LazyStellarValue as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 84;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `id_pool`.
    #[must_use]
    pub fn id_pool(&self) -> u64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 36;
        pos += <LazyStellarValue as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 88;
        <u64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `base_fee`.
    #[must_use]
    pub fn base_fee(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 36;
        pos += <LazyStellarValue as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 96;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `base_reserve`.
    #[must_use]
    pub fn base_reserve(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 36;
        pos += <LazyStellarValue as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 100;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `max_tx_set_size`.
    #[must_use]
    pub fn max_tx_set_size(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 36;
        pos += <LazyStellarValue as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 104;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `skip_list`.
    #[must_use]
    pub fn skip_list(&self) -> LazyFixedArray<LazyHash, 4> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 36;
        pos += <LazyStellarValue as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 108;
        <LazyFixedArray<LazyHash, 4> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazyLedgerHeaderExt {
        let buf = self.0.as_slice();
        let mut pos: u32 = 36;
        pos += <LazyStellarValue as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 236;
        <LazyLedgerHeaderExt as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LedgerHeader> for LazyLedgerHeader {
    type Error = Error;
    fn try_from(val: &LedgerHeader) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyLedgerHeader> for LedgerHeader {
    type Error = Error;
    fn try_from(lazy: &LazyLedgerHeader) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
