#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ConfigSettingContractComputeV0 is an XDR Struct defined as:
///
/// ```text
/// struct ConfigSettingContractComputeV0
/// {
///     // Maximum instructions per ledger
///     int64 ledgerMaxInstructions;
///     // Maximum instructions per transaction
///     int64 txMaxInstructions;
///     // Cost of 10000 instructions
///     int64 feeRatePerInstructionsIncrement;
///
///     // Memory limit per transaction. Unlike instructions, there is no fee
///     // for memory, just the limit.
///     uint32 txMemoryLimit;
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
pub struct ConfigSettingContractComputeV0 {
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub ledger_max_instructions: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub tx_max_instructions: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub fee_rate_per_instructions_increment: i64,
    pub tx_memory_limit: u32,
}

impl ReadXdr for ConfigSettingContractComputeV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ledger_max_instructions: i64::read_xdr(r)?,
                tx_max_instructions: i64::read_xdr(r)?,
                fee_rate_per_instructions_increment: i64::read_xdr(r)?,
                tx_memory_limit: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ConfigSettingContractComputeV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_max_instructions.write_xdr(w)?;
            self.tx_max_instructions.write_xdr(w)?;
            self.fee_rate_per_instructions_increment.write_xdr(w)?;
            self.tx_memory_limit.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ConfigSettingContractComputeV0`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyConfigSettingContractComputeV0(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyConfigSettingContractComputeV0 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyConfigSettingContractComputeV0 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| {
                self.ledger_max_instructions()
                    .cmp(&other.ledger_max_instructions())
            })
            .then_with(|| self.tx_max_instructions().cmp(&other.tx_max_instructions()))
            .then_with(|| {
                self.fee_rate_per_instructions_increment()
                    .cmp(&other.fee_rate_per_instructions_increment())
            })
            .then_with(|| self.tx_memory_limit().cmp(&other.tx_memory_limit()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyConfigSettingContractComputeV0 {
    const FIXED_XDR_SIZE: Option<u32> = Some(28);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(28).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        28
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
impl From<LazyHandle> for LazyConfigSettingContractComputeV0 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyConfigSettingContractComputeV0 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyConfigSettingContractComputeV0 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyConfigSettingContractComputeV0 {
    /// Access field `ledger_max_instructions`.
    #[must_use]
    pub fn ledger_max_instructions(&self) -> i64 {
        <i64 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `tx_max_instructions`.
    #[must_use]
    pub fn tx_max_instructions(&self) -> i64 {
        <i64 as LazyXdr>::from_xdr_at(&self.0, 8)
    }
    /// Access field `fee_rate_per_instructions_increment`.
    #[must_use]
    pub fn fee_rate_per_instructions_increment(&self) -> i64 {
        <i64 as LazyXdr>::from_xdr_at(&self.0, 16)
    }
    /// Access field `tx_memory_limit`.
    #[must_use]
    pub fn tx_memory_limit(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 24)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ConfigSettingContractComputeV0> for LazyConfigSettingContractComputeV0 {
    type Error = Error;
    fn try_from(val: &ConfigSettingContractComputeV0) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyConfigSettingContractComputeV0> for ConfigSettingContractComputeV0 {
    type Error = Error;
    fn try_from(lazy: &LazyConfigSettingContractComputeV0) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
