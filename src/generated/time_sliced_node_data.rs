#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// TimeSlicedNodeData is an XDR Struct defined as:
///
/// ```text
/// struct TimeSlicedNodeData
/// {
///     uint32 addedAuthenticatedPeers;
///     uint32 droppedAuthenticatedPeers;
///     uint32 totalInboundPeerCount;
///     uint32 totalOutboundPeerCount;
///
///     // SCP stats
///     uint32 p75SCPFirstToSelfLatencyMs;
///     uint32 p75SCPSelfToOtherLatencyMs;
///
///     // How many times the node lost sync in the time slice
///     uint32 lostSyncCount;
///
///     // Config data
///     bool isValidator;
///     uint32 maxInboundPeerCount;
///     uint32 maxOutboundPeerCount;
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
pub struct TimeSlicedNodeData {
    pub added_authenticated_peers: u32,
    pub dropped_authenticated_peers: u32,
    pub total_inbound_peer_count: u32,
    pub total_outbound_peer_count: u32,
    pub p75_scp_first_to_self_latency_ms: u32,
    pub p75_scp_self_to_other_latency_ms: u32,
    pub lost_sync_count: u32,
    pub is_validator: bool,
    pub max_inbound_peer_count: u32,
    pub max_outbound_peer_count: u32,
}

impl ReadXdr for TimeSlicedNodeData {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                added_authenticated_peers: u32::read_xdr(r)?,
                dropped_authenticated_peers: u32::read_xdr(r)?,
                total_inbound_peer_count: u32::read_xdr(r)?,
                total_outbound_peer_count: u32::read_xdr(r)?,
                p75_scp_first_to_self_latency_ms: u32::read_xdr(r)?,
                p75_scp_self_to_other_latency_ms: u32::read_xdr(r)?,
                lost_sync_count: u32::read_xdr(r)?,
                is_validator: bool::read_xdr(r)?,
                max_inbound_peer_count: u32::read_xdr(r)?,
                max_outbound_peer_count: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TimeSlicedNodeData {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.added_authenticated_peers.write_xdr(w)?;
            self.dropped_authenticated_peers.write_xdr(w)?;
            self.total_inbound_peer_count.write_xdr(w)?;
            self.total_outbound_peer_count.write_xdr(w)?;
            self.p75_scp_first_to_self_latency_ms.write_xdr(w)?;
            self.p75_scp_self_to_other_latency_ms.write_xdr(w)?;
            self.lost_sync_count.write_xdr(w)?;
            self.is_validator.write_xdr(w)?;
            self.max_inbound_peer_count.write_xdr(w)?;
            self.max_outbound_peer_count.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`TimeSlicedNodeData`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyTimeSlicedNodeData(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyTimeSlicedNodeData {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyTimeSlicedNodeData {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| {
                self.added_authenticated_peers()
                    .cmp(&other.added_authenticated_peers())
            })
            .then_with(|| {
                self.dropped_authenticated_peers()
                    .cmp(&other.dropped_authenticated_peers())
            })
            .then_with(|| {
                self.total_inbound_peer_count()
                    .cmp(&other.total_inbound_peer_count())
            })
            .then_with(|| {
                self.total_outbound_peer_count()
                    .cmp(&other.total_outbound_peer_count())
            })
            .then_with(|| {
                self.p75_scp_first_to_self_latency_ms()
                    .cmp(&other.p75_scp_first_to_self_latency_ms())
            })
            .then_with(|| {
                self.p75_scp_self_to_other_latency_ms()
                    .cmp(&other.p75_scp_self_to_other_latency_ms())
            })
            .then_with(|| self.lost_sync_count().cmp(&other.lost_sync_count()))
            .then_with(|| self.is_validator().cmp(&other.is_validator()))
            .then_with(|| {
                self.max_inbound_peer_count()
                    .cmp(&other.max_inbound_peer_count())
            })
            .then_with(|| {
                self.max_outbound_peer_count()
                    .cmp(&other.max_outbound_peer_count())
            })
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyTimeSlicedNodeData {
    const FIXED_XDR_SIZE: Option<u32> = Some(40);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(40).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <bool as LazyXdr>::xdr_validate(&buf[(pos + 28) as usize..], depth)?;
        pos = next_pos;
        Ok(pos)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        40
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyTimeSlicedNodeData {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyTimeSlicedNodeData {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyTimeSlicedNodeData {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyTimeSlicedNodeData {
    /// Access field `added_authenticated_peers`.
    #[must_use]
    pub fn added_authenticated_peers(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `dropped_authenticated_peers`.
    #[must_use]
    pub fn dropped_authenticated_peers(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 4)
    }
    /// Access field `total_inbound_peer_count`.
    #[must_use]
    pub fn total_inbound_peer_count(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 8)
    }
    /// Access field `total_outbound_peer_count`.
    #[must_use]
    pub fn total_outbound_peer_count(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 12)
    }
    /// Access field `p75_scp_first_to_self_latency_ms`.
    #[must_use]
    pub fn p75_scp_first_to_self_latency_ms(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 16)
    }
    /// Access field `p75_scp_self_to_other_latency_ms`.
    #[must_use]
    pub fn p75_scp_self_to_other_latency_ms(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 20)
    }
    /// Access field `lost_sync_count`.
    #[must_use]
    pub fn lost_sync_count(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 24)
    }
    /// Access field `is_validator`.
    #[must_use]
    pub fn is_validator(&self) -> bool {
        <bool as LazyXdr>::from_xdr_at(&self.0, 28)
    }
    /// Access field `max_inbound_peer_count`.
    #[must_use]
    pub fn max_inbound_peer_count(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 32)
    }
    /// Access field `max_outbound_peer_count`.
    #[must_use]
    pub fn max_outbound_peer_count(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 36)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&TimeSlicedNodeData> for LazyTimeSlicedNodeData {
    type Error = Error;
    fn try_from(val: &TimeSlicedNodeData) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyTimeSlicedNodeData> for TimeSlicedNodeData {
    type Error = Error;
    fn try_from(lazy: &LazyTimeSlicedNodeData) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
