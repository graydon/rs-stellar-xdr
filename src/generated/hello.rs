#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// Hello is an XDR Struct defined as:
///
/// ```text
/// struct Hello
/// {
///     uint32 ledgerVersion;
///     uint32 overlayVersion;
///     uint32 overlayMinVersion;
///     Hash networkID;
///     string versionStr<100>;
///     int listeningPort;
///     NodeID peerID;
///     AuthCert cert;
///     uint256 nonce;
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
pub struct Hello {
    pub ledger_version: u32,
    pub overlay_version: u32,
    pub overlay_min_version: u32,
    pub network_id: Hash,
    pub version_str: StringM<100>,
    pub listening_port: i32,
    pub peer_id: NodeId,
    pub cert: AuthCert,
    pub nonce: Uint256,
}

impl ReadXdr for Hello {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ledger_version: u32::read_xdr(r)?,
                overlay_version: u32::read_xdr(r)?,
                overlay_min_version: u32::read_xdr(r)?,
                network_id: Hash::read_xdr(r)?,
                version_str: StringM::<100>::read_xdr(r)?,
                listening_port: i32::read_xdr(r)?,
                peer_id: NodeId::read_xdr(r)?,
                cert: AuthCert::read_xdr(r)?,
                nonce: Uint256::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for Hello {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_version.write_xdr(w)?;
            self.overlay_version.write_xdr(w)?;
            self.overlay_min_version.write_xdr(w)?;
            self.network_id.write_xdr(w)?;
            self.version_str.write_xdr(w)?;
            self.listening_port.write_xdr(w)?;
            self.peer_id.write_xdr(w)?;
            self.cert.write_xdr(w)?;
            self.nonce.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`Hello`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyHello(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyHello {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyHello {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.ledger_version().cmp(&other.ledger_version()))
            .then_with(|| self.overlay_version().cmp(&other.overlay_version()))
            .then_with(|| self.overlay_min_version().cmp(&other.overlay_min_version()))
            .then_with(|| self.network_id().cmp(&other.network_id()))
            .then_with(|| self.version_str().cmp(&other.version_str()))
            .then_with(|| self.listening_port().cmp(&other.listening_port()))
            .then_with(|| self.peer_id().cmp(&other.peer_id()))
            .then_with(|| self.cert().cmp(&other.cert()))
            .then_with(|| self.nonce().cmp(&other.nonce()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyHello {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(44).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyHash as LazyXdr>::xdr_validate(&buf[(pos + 12) as usize..], depth)?;
        pos = next_pos;
        {
            let field_len =
                <LazyStringM<100> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(4).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        {
            let field_len = <LazyNodeId as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyAuthCert as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(32).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyUint256 as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += 44;
        pos += <LazyStringM<100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyAuthCert as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 32;
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
impl From<LazyHandle> for LazyHello {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyHello {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyHello {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyHello {
    /// Access field `ledger_version`.
    #[must_use]
    pub fn ledger_version(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `overlay_version`.
    #[must_use]
    pub fn overlay_version(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 4)
    }
    /// Access field `overlay_min_version`.
    #[must_use]
    pub fn overlay_min_version(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 8)
    }
    /// Access field `network_id`.
    #[must_use]
    pub fn network_id(&self) -> LazyHash {
        <LazyHash as LazyXdr>::from_xdr_at(&self.0, 12)
    }
    /// Access field `version_str`.
    #[must_use]
    pub fn version_str(&self) -> LazyStringM<100> {
        <LazyStringM<100> as LazyXdr>::from_xdr_at(&self.0, 44)
    }
    /// Access field `listening_port`.
    #[must_use]
    pub fn listening_port(&self) -> i32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 44;
        pos += <LazyStringM<100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <i32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `peer_id`.
    #[must_use]
    pub fn peer_id(&self) -> LazyNodeId {
        let buf = self.0.as_slice();
        let mut pos: u32 = 44;
        pos += <LazyStringM<100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        <LazyNodeId as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `cert`.
    #[must_use]
    pub fn cert(&self) -> LazyAuthCert {
        let buf = self.0.as_slice();
        let mut pos: u32 = 44;
        pos += <LazyStringM<100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyAuthCert as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `nonce`.
    #[must_use]
    pub fn nonce(&self) -> LazyUint256 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 44;
        pos += <LazyStringM<100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyAuthCert as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyUint256 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&Hello> for LazyHello {
    type Error = Error;
    fn try_from(val: &Hello) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyHello> for Hello {
    type Error = Error;
    fn try_from(lazy: &LazyHello) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
