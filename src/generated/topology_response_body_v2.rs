#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// TopologyResponseBodyV2 is an XDR Struct defined as:
///
/// ```text
/// struct TopologyResponseBodyV2
/// {
///     TimeSlicedPeerDataList inboundPeers;
///     TimeSlicedPeerDataList outboundPeers;
///     TimeSlicedNodeData nodeData;
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
pub struct TopologyResponseBodyV2 {
    pub inbound_peers: TimeSlicedPeerDataList,
    pub outbound_peers: TimeSlicedPeerDataList,
    pub node_data: TimeSlicedNodeData,
}

impl ReadXdr for TopologyResponseBodyV2 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                inbound_peers: TimeSlicedPeerDataList::read_xdr(r)?,
                outbound_peers: TimeSlicedPeerDataList::read_xdr(r)?,
                node_data: TimeSlicedNodeData::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TopologyResponseBodyV2 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.inbound_peers.write_xdr(w)?;
            self.outbound_peers.write_xdr(w)?;
            self.node_data.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`TopologyResponseBodyV2`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyTopologyResponseBodyV2(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyTopologyResponseBodyV2 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyTopologyResponseBodyV2 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.inbound_peers().cmp(&other.inbound_peers()))
            .then_with(|| self.outbound_peers().cmp(&other.outbound_peers()))
            .then_with(|| self.node_data().cmp(&other.node_data()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyTopologyResponseBodyV2 {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyTimeSlicedPeerDataList as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyTimeSlicedPeerDataList as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(40).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyTimeSlicedNodeData as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyTimeSlicedPeerDataList as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyTimeSlicedPeerDataList as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 40;
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
impl From<LazyHandle> for LazyTopologyResponseBodyV2 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyTopologyResponseBodyV2 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyTopologyResponseBodyV2 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyTopologyResponseBodyV2 {
    /// Access field `inbound_peers`.
    #[must_use]
    pub fn inbound_peers(&self) -> LazyTimeSlicedPeerDataList {
        <LazyTimeSlicedPeerDataList as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `outbound_peers`.
    #[must_use]
    pub fn outbound_peers(&self) -> LazyTimeSlicedPeerDataList {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyTimeSlicedPeerDataList as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyTimeSlicedPeerDataList as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `node_data`.
    #[must_use]
    pub fn node_data(&self) -> LazyTimeSlicedNodeData {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyTimeSlicedPeerDataList as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyTimeSlicedPeerDataList as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyTimeSlicedNodeData as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&TopologyResponseBodyV2> for LazyTopologyResponseBodyV2 {
    type Error = Error;
    fn try_from(val: &TopologyResponseBodyV2) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyTopologyResponseBodyV2> for TopologyResponseBodyV2 {
    type Error = Error;
    fn try_from(lazy: &LazyTopologyResponseBodyV2) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
