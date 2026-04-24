#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// PeerStats is an XDR Struct defined as:
///
/// ```text
/// struct PeerStats
/// {
///     NodeID id;
///     string versionStr<100>;
///     uint64 messagesRead;
///     uint64 messagesWritten;
///     uint64 bytesRead;
///     uint64 bytesWritten;
///     uint64 secondsConnected;
///
///     uint64 uniqueFloodBytesRecv;
///     uint64 duplicateFloodBytesRecv;
///     uint64 uniqueFetchBytesRecv;
///     uint64 duplicateFetchBytesRecv;
///
///     uint64 uniqueFloodMessageRecv;
///     uint64 duplicateFloodMessageRecv;
///     uint64 uniqueFetchMessageRecv;
///     uint64 duplicateFetchMessageRecv;
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
pub struct PeerStats {
    pub id: NodeId,
    pub version_str: StringM<100>,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub messages_read: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub messages_written: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub bytes_read: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub bytes_written: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub seconds_connected: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub unique_flood_bytes_recv: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub duplicate_flood_bytes_recv: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub unique_fetch_bytes_recv: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub duplicate_fetch_bytes_recv: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub unique_flood_message_recv: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub duplicate_flood_message_recv: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub unique_fetch_message_recv: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub duplicate_fetch_message_recv: u64,
}

impl ReadXdr for PeerStats {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                id: NodeId::read_xdr(r)?,
                version_str: StringM::<100>::read_xdr(r)?,
                messages_read: u64::read_xdr(r)?,
                messages_written: u64::read_xdr(r)?,
                bytes_read: u64::read_xdr(r)?,
                bytes_written: u64::read_xdr(r)?,
                seconds_connected: u64::read_xdr(r)?,
                unique_flood_bytes_recv: u64::read_xdr(r)?,
                duplicate_flood_bytes_recv: u64::read_xdr(r)?,
                unique_fetch_bytes_recv: u64::read_xdr(r)?,
                duplicate_fetch_bytes_recv: u64::read_xdr(r)?,
                unique_flood_message_recv: u64::read_xdr(r)?,
                duplicate_flood_message_recv: u64::read_xdr(r)?,
                unique_fetch_message_recv: u64::read_xdr(r)?,
                duplicate_fetch_message_recv: u64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for PeerStats {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.id.write_xdr(w)?;
            self.version_str.write_xdr(w)?;
            self.messages_read.write_xdr(w)?;
            self.messages_written.write_xdr(w)?;
            self.bytes_read.write_xdr(w)?;
            self.bytes_written.write_xdr(w)?;
            self.seconds_connected.write_xdr(w)?;
            self.unique_flood_bytes_recv.write_xdr(w)?;
            self.duplicate_flood_bytes_recv.write_xdr(w)?;
            self.unique_fetch_bytes_recv.write_xdr(w)?;
            self.duplicate_fetch_bytes_recv.write_xdr(w)?;
            self.unique_flood_message_recv.write_xdr(w)?;
            self.duplicate_flood_message_recv.write_xdr(w)?;
            self.unique_fetch_message_recv.write_xdr(w)?;
            self.duplicate_fetch_message_recv.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`PeerStats`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyPeerStats(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyPeerStats {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyPeerStats {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.id().cmp(&other.id()))
            .then_with(|| self.version_str().cmp(&other.version_str()))
            .then_with(|| self.messages_read().cmp(&other.messages_read()))
            .then_with(|| self.messages_written().cmp(&other.messages_written()))
            .then_with(|| self.bytes_read().cmp(&other.bytes_read()))
            .then_with(|| self.bytes_written().cmp(&other.bytes_written()))
            .then_with(|| self.seconds_connected().cmp(&other.seconds_connected()))
            .then_with(|| {
                self.unique_flood_bytes_recv()
                    .cmp(&other.unique_flood_bytes_recv())
            })
            .then_with(|| {
                self.duplicate_flood_bytes_recv()
                    .cmp(&other.duplicate_flood_bytes_recv())
            })
            .then_with(|| {
                self.unique_fetch_bytes_recv()
                    .cmp(&other.unique_fetch_bytes_recv())
            })
            .then_with(|| {
                self.duplicate_fetch_bytes_recv()
                    .cmp(&other.duplicate_fetch_bytes_recv())
            })
            .then_with(|| {
                self.unique_flood_message_recv()
                    .cmp(&other.unique_flood_message_recv())
            })
            .then_with(|| {
                self.duplicate_flood_message_recv()
                    .cmp(&other.duplicate_flood_message_recv())
            })
            .then_with(|| {
                self.unique_fetch_message_recv()
                    .cmp(&other.unique_fetch_message_recv())
            })
            .then_with(|| {
                self.duplicate_fetch_message_recv()
                    .cmp(&other.duplicate_fetch_message_recv())
            })
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyPeerStats {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazyNodeId as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyStringM<100> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(104).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 104;
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
impl From<LazyHandle> for LazyPeerStats {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyPeerStats {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyPeerStats {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyPeerStats {
    /// Access field `id`.
    #[must_use]
    pub fn id(&self) -> LazyNodeId {
        <LazyNodeId as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `version_str`.
    #[must_use]
    pub fn version_str(&self) -> LazyStringM<100> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyStringM<100> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `messages_read`.
    #[must_use]
    pub fn messages_read(&self) -> u64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <u64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `messages_written`.
    #[must_use]
    pub fn messages_written(&self) -> u64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        <u64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `bytes_read`.
    #[must_use]
    pub fn bytes_read(&self) -> u64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 16;
        <u64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `bytes_written`.
    #[must_use]
    pub fn bytes_written(&self) -> u64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 24;
        <u64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `seconds_connected`.
    #[must_use]
    pub fn seconds_connected(&self) -> u64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 32;
        <u64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `unique_flood_bytes_recv`.
    #[must_use]
    pub fn unique_flood_bytes_recv(&self) -> u64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 40;
        <u64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `duplicate_flood_bytes_recv`.
    #[must_use]
    pub fn duplicate_flood_bytes_recv(&self) -> u64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 48;
        <u64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `unique_fetch_bytes_recv`.
    #[must_use]
    pub fn unique_fetch_bytes_recv(&self) -> u64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 56;
        <u64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `duplicate_fetch_bytes_recv`.
    #[must_use]
    pub fn duplicate_fetch_bytes_recv(&self) -> u64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 64;
        <u64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `unique_flood_message_recv`.
    #[must_use]
    pub fn unique_flood_message_recv(&self) -> u64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 72;
        <u64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `duplicate_flood_message_recv`.
    #[must_use]
    pub fn duplicate_flood_message_recv(&self) -> u64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 80;
        <u64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `unique_fetch_message_recv`.
    #[must_use]
    pub fn unique_fetch_message_recv(&self) -> u64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 88;
        <u64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `duplicate_fetch_message_recv`.
    #[must_use]
    pub fn duplicate_fetch_message_recv(&self) -> u64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 96;
        <u64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&PeerStats> for LazyPeerStats {
    type Error = Error;
    fn try_from(val: &PeerStats) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyPeerStats> for PeerStats {
    type Error = Error;
    fn try_from(lazy: &LazyPeerStats) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
