#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ConfigSettingContractEventsV0 is an XDR Struct defined as:
///
/// ```text
/// struct ConfigSettingContractEventsV0
/// {
///     // Maximum size of events that a contract call can emit.
///     uint32 txMaxContractEventsSizeBytes;
///     // Fee for generating 1KB of contract events.
///     int64 feeContractEvents1KB;
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
pub struct ConfigSettingContractEventsV0 {
    pub tx_max_contract_events_size_bytes: u32,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub fee_contract_events1_kb: i64,
}

impl ReadXdr for ConfigSettingContractEventsV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                tx_max_contract_events_size_bytes: u32::read_xdr(r)?,
                fee_contract_events1_kb: i64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ConfigSettingContractEventsV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx_max_contract_events_size_bytes.write_xdr(w)?;
            self.fee_contract_events1_kb.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ConfigSettingContractEventsV0`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyConfigSettingContractEventsV0(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyConfigSettingContractEventsV0 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyConfigSettingContractEventsV0 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| {
                self.tx_max_contract_events_size_bytes()
                    .cmp(&other.tx_max_contract_events_size_bytes())
            })
            .then_with(|| {
                self.fee_contract_events1_kb()
                    .cmp(&other.fee_contract_events1_kb())
            })
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyConfigSettingContractEventsV0 {
    const FIXED_XDR_SIZE: Option<u32> = Some(12);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(12).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        12
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
impl From<LazyHandle> for LazyConfigSettingContractEventsV0 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyConfigSettingContractEventsV0 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyConfigSettingContractEventsV0 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyConfigSettingContractEventsV0 {
    /// Access field `tx_max_contract_events_size_bytes`.
    #[must_use]
    pub fn tx_max_contract_events_size_bytes(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `fee_contract_events1_kb`.
    #[must_use]
    pub fn fee_contract_events1_kb(&self) -> i64 {
        <i64 as LazyXdr>::from_xdr_at(&self.0, 4)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ConfigSettingContractEventsV0> for LazyConfigSettingContractEventsV0 {
    type Error = Error;
    fn try_from(val: &ConfigSettingContractEventsV0) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyConfigSettingContractEventsV0> for ConfigSettingContractEventsV0 {
    type Error = Error;
    fn try_from(lazy: &LazyConfigSettingContractEventsV0) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
