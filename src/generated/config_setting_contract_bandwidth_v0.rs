#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ConfigSettingContractBandwidthV0 is an XDR Struct defined as:
///
/// ```text
/// struct ConfigSettingContractBandwidthV0
/// {
///     // Maximum sum of all transaction sizes in the ledger in bytes
///     uint32 ledgerMaxTxsSizeBytes;
///     // Maximum size in bytes for a transaction
///     uint32 txMaxSizeBytes;
///
///     // Fee for 1 KB of transaction size
///     int64 feeTxSize1KB;
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
pub struct ConfigSettingContractBandwidthV0 {
    pub ledger_max_txs_size_bytes: u32,
    pub tx_max_size_bytes: u32,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub fee_tx_size1_kb: i64,
}

impl ReadXdr for ConfigSettingContractBandwidthV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ledger_max_txs_size_bytes: u32::read_xdr(r)?,
                tx_max_size_bytes: u32::read_xdr(r)?,
                fee_tx_size1_kb: i64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ConfigSettingContractBandwidthV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_max_txs_size_bytes.write_xdr(w)?;
            self.tx_max_size_bytes.write_xdr(w)?;
            self.fee_tx_size1_kb.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ConfigSettingContractBandwidthV0`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyConfigSettingContractBandwidthV0(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyConfigSettingContractBandwidthV0 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyConfigSettingContractBandwidthV0 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| {
                self.ledger_max_txs_size_bytes()
                    .cmp(&other.ledger_max_txs_size_bytes())
            })
            .then_with(|| self.tx_max_size_bytes().cmp(&other.tx_max_size_bytes()))
            .then_with(|| self.fee_tx_size1_kb().cmp(&other.fee_tx_size1_kb()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyConfigSettingContractBandwidthV0 {
    const FIXED_XDR_SIZE: Option<u32> = Some(16);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(16).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        16
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
impl From<LazyHandle> for LazyConfigSettingContractBandwidthV0 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyConfigSettingContractBandwidthV0 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyConfigSettingContractBandwidthV0 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyConfigSettingContractBandwidthV0 {
    /// Access field `ledger_max_txs_size_bytes`.
    #[must_use]
    pub fn ledger_max_txs_size_bytes(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `tx_max_size_bytes`.
    #[must_use]
    pub fn tx_max_size_bytes(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 4)
    }
    /// Access field `fee_tx_size1_kb`.
    #[must_use]
    pub fn fee_tx_size1_kb(&self) -> i64 {
        <i64 as LazyXdr>::from_xdr_at(&self.0, 8)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ConfigSettingContractBandwidthV0> for LazyConfigSettingContractBandwidthV0 {
    type Error = Error;
    fn try_from(val: &ConfigSettingContractBandwidthV0) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyConfigSettingContractBandwidthV0> for ConfigSettingContractBandwidthV0 {
    type Error = Error;
    fn try_from(lazy: &LazyConfigSettingContractBandwidthV0) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
