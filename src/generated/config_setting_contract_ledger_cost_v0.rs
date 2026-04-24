#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ConfigSettingContractLedgerCostV0 is an XDR Struct defined as:
///
/// ```text
/// struct ConfigSettingContractLedgerCostV0
/// {
///     // Maximum number of disk entry read operations per ledger
///     uint32 ledgerMaxDiskReadEntries;
///     // Maximum number of bytes of disk reads that can be performed per ledger
///     uint32 ledgerMaxDiskReadBytes;
///     // Maximum number of ledger entry write operations per ledger
///     uint32 ledgerMaxWriteLedgerEntries;
///     // Maximum number of bytes that can be written per ledger
///     uint32 ledgerMaxWriteBytes;
///
///     // Maximum number of disk entry read operations per transaction
///     uint32 txMaxDiskReadEntries;
///     // Maximum number of bytes of disk reads that can be performed per transaction
///     uint32 txMaxDiskReadBytes;
///     // Maximum number of ledger entry write operations per transaction
///     uint32 txMaxWriteLedgerEntries;
///     // Maximum number of bytes that can be written per transaction
///     uint32 txMaxWriteBytes;
///
///     int64 feeDiskReadLedgerEntry;  // Fee per disk ledger entry read
///     int64 feeWriteLedgerEntry;     // Fee per ledger entry write
///
///     int64 feeDiskRead1KB;          // Fee for reading 1KB disk
///
///     // The following parameters determine the write fee per 1KB.
///     // Rent fee grows linearly until soroban state reaches this size
///     int64 sorobanStateTargetSizeBytes;
///     // Fee per 1KB rent when the soroban state is empty
///     int64 rentFee1KBSorobanStateSizeLow;
///     // Fee per 1KB rent when the soroban state has reached `sorobanStateTargetSizeBytes`
///     int64 rentFee1KBSorobanStateSizeHigh;
///     // Rent fee multiplier for any additional data past the first `sorobanStateTargetSizeBytes`
///     uint32 sorobanStateRentFeeGrowthFactor;
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
pub struct ConfigSettingContractLedgerCostV0 {
    pub ledger_max_disk_read_entries: u32,
    pub ledger_max_disk_read_bytes: u32,
    pub ledger_max_write_ledger_entries: u32,
    pub ledger_max_write_bytes: u32,
    pub tx_max_disk_read_entries: u32,
    pub tx_max_disk_read_bytes: u32,
    pub tx_max_write_ledger_entries: u32,
    pub tx_max_write_bytes: u32,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub fee_disk_read_ledger_entry: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub fee_write_ledger_entry: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub fee_disk_read1_kb: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub soroban_state_target_size_bytes: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub rent_fee1_kb_soroban_state_size_low: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub rent_fee1_kb_soroban_state_size_high: i64,
    pub soroban_state_rent_fee_growth_factor: u32,
}

impl ReadXdr for ConfigSettingContractLedgerCostV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ledger_max_disk_read_entries: u32::read_xdr(r)?,
                ledger_max_disk_read_bytes: u32::read_xdr(r)?,
                ledger_max_write_ledger_entries: u32::read_xdr(r)?,
                ledger_max_write_bytes: u32::read_xdr(r)?,
                tx_max_disk_read_entries: u32::read_xdr(r)?,
                tx_max_disk_read_bytes: u32::read_xdr(r)?,
                tx_max_write_ledger_entries: u32::read_xdr(r)?,
                tx_max_write_bytes: u32::read_xdr(r)?,
                fee_disk_read_ledger_entry: i64::read_xdr(r)?,
                fee_write_ledger_entry: i64::read_xdr(r)?,
                fee_disk_read1_kb: i64::read_xdr(r)?,
                soroban_state_target_size_bytes: i64::read_xdr(r)?,
                rent_fee1_kb_soroban_state_size_low: i64::read_xdr(r)?,
                rent_fee1_kb_soroban_state_size_high: i64::read_xdr(r)?,
                soroban_state_rent_fee_growth_factor: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ConfigSettingContractLedgerCostV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_max_disk_read_entries.write_xdr(w)?;
            self.ledger_max_disk_read_bytes.write_xdr(w)?;
            self.ledger_max_write_ledger_entries.write_xdr(w)?;
            self.ledger_max_write_bytes.write_xdr(w)?;
            self.tx_max_disk_read_entries.write_xdr(w)?;
            self.tx_max_disk_read_bytes.write_xdr(w)?;
            self.tx_max_write_ledger_entries.write_xdr(w)?;
            self.tx_max_write_bytes.write_xdr(w)?;
            self.fee_disk_read_ledger_entry.write_xdr(w)?;
            self.fee_write_ledger_entry.write_xdr(w)?;
            self.fee_disk_read1_kb.write_xdr(w)?;
            self.soroban_state_target_size_bytes.write_xdr(w)?;
            self.rent_fee1_kb_soroban_state_size_low.write_xdr(w)?;
            self.rent_fee1_kb_soroban_state_size_high.write_xdr(w)?;
            self.soroban_state_rent_fee_growth_factor.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ConfigSettingContractLedgerCostV0`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyConfigSettingContractLedgerCostV0(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyConfigSettingContractLedgerCostV0 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyConfigSettingContractLedgerCostV0 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| {
                self.ledger_max_disk_read_entries()
                    .cmp(&other.ledger_max_disk_read_entries())
            })
            .then_with(|| {
                self.ledger_max_disk_read_bytes()
                    .cmp(&other.ledger_max_disk_read_bytes())
            })
            .then_with(|| {
                self.ledger_max_write_ledger_entries()
                    .cmp(&other.ledger_max_write_ledger_entries())
            })
            .then_with(|| {
                self.ledger_max_write_bytes()
                    .cmp(&other.ledger_max_write_bytes())
            })
            .then_with(|| {
                self.tx_max_disk_read_entries()
                    .cmp(&other.tx_max_disk_read_entries())
            })
            .then_with(|| {
                self.tx_max_disk_read_bytes()
                    .cmp(&other.tx_max_disk_read_bytes())
            })
            .then_with(|| {
                self.tx_max_write_ledger_entries()
                    .cmp(&other.tx_max_write_ledger_entries())
            })
            .then_with(|| self.tx_max_write_bytes().cmp(&other.tx_max_write_bytes()))
            .then_with(|| {
                self.fee_disk_read_ledger_entry()
                    .cmp(&other.fee_disk_read_ledger_entry())
            })
            .then_with(|| {
                self.fee_write_ledger_entry()
                    .cmp(&other.fee_write_ledger_entry())
            })
            .then_with(|| self.fee_disk_read1_kb().cmp(&other.fee_disk_read1_kb()))
            .then_with(|| {
                self.soroban_state_target_size_bytes()
                    .cmp(&other.soroban_state_target_size_bytes())
            })
            .then_with(|| {
                self.rent_fee1_kb_soroban_state_size_low()
                    .cmp(&other.rent_fee1_kb_soroban_state_size_low())
            })
            .then_with(|| {
                self.rent_fee1_kb_soroban_state_size_high()
                    .cmp(&other.rent_fee1_kb_soroban_state_size_high())
            })
            .then_with(|| {
                self.soroban_state_rent_fee_growth_factor()
                    .cmp(&other.soroban_state_rent_fee_growth_factor())
            })
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyConfigSettingContractLedgerCostV0 {
    const FIXED_XDR_SIZE: Option<u32> = Some(84);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(84).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        84
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
impl From<LazyHandle> for LazyConfigSettingContractLedgerCostV0 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyConfigSettingContractLedgerCostV0 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyConfigSettingContractLedgerCostV0 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyConfigSettingContractLedgerCostV0 {
    /// Access field `ledger_max_disk_read_entries`.
    #[must_use]
    pub fn ledger_max_disk_read_entries(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `ledger_max_disk_read_bytes`.
    #[must_use]
    pub fn ledger_max_disk_read_bytes(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 4)
    }
    /// Access field `ledger_max_write_ledger_entries`.
    #[must_use]
    pub fn ledger_max_write_ledger_entries(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 8)
    }
    /// Access field `ledger_max_write_bytes`.
    #[must_use]
    pub fn ledger_max_write_bytes(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 12)
    }
    /// Access field `tx_max_disk_read_entries`.
    #[must_use]
    pub fn tx_max_disk_read_entries(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 16)
    }
    /// Access field `tx_max_disk_read_bytes`.
    #[must_use]
    pub fn tx_max_disk_read_bytes(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 20)
    }
    /// Access field `tx_max_write_ledger_entries`.
    #[must_use]
    pub fn tx_max_write_ledger_entries(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 24)
    }
    /// Access field `tx_max_write_bytes`.
    #[must_use]
    pub fn tx_max_write_bytes(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 28)
    }
    /// Access field `fee_disk_read_ledger_entry`.
    #[must_use]
    pub fn fee_disk_read_ledger_entry(&self) -> i64 {
        <i64 as LazyXdr>::from_xdr_at(&self.0, 32)
    }
    /// Access field `fee_write_ledger_entry`.
    #[must_use]
    pub fn fee_write_ledger_entry(&self) -> i64 {
        <i64 as LazyXdr>::from_xdr_at(&self.0, 40)
    }
    /// Access field `fee_disk_read1_kb`.
    #[must_use]
    pub fn fee_disk_read1_kb(&self) -> i64 {
        <i64 as LazyXdr>::from_xdr_at(&self.0, 48)
    }
    /// Access field `soroban_state_target_size_bytes`.
    #[must_use]
    pub fn soroban_state_target_size_bytes(&self) -> i64 {
        <i64 as LazyXdr>::from_xdr_at(&self.0, 56)
    }
    /// Access field `rent_fee1_kb_soroban_state_size_low`.
    #[must_use]
    pub fn rent_fee1_kb_soroban_state_size_low(&self) -> i64 {
        <i64 as LazyXdr>::from_xdr_at(&self.0, 64)
    }
    /// Access field `rent_fee1_kb_soroban_state_size_high`.
    #[must_use]
    pub fn rent_fee1_kb_soroban_state_size_high(&self) -> i64 {
        <i64 as LazyXdr>::from_xdr_at(&self.0, 72)
    }
    /// Access field `soroban_state_rent_fee_growth_factor`.
    #[must_use]
    pub fn soroban_state_rent_fee_growth_factor(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 80)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ConfigSettingContractLedgerCostV0> for LazyConfigSettingContractLedgerCostV0 {
    type Error = Error;
    fn try_from(val: &ConfigSettingContractLedgerCostV0) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyConfigSettingContractLedgerCostV0> for ConfigSettingContractLedgerCostV0 {
    type Error = Error;
    fn try_from(lazy: &LazyConfigSettingContractLedgerCostV0) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
