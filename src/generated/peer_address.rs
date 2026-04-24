#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// PeerAddress is an XDR Struct defined as:
///
/// ```text
/// struct PeerAddress
/// {
///     union switch (IPAddrType type)
///     {
///     case IPv4:
///         opaque ipv4[4];
///     case IPv6:
///         opaque ipv6[16];
///     }
///     ip;
///     uint32 port;
///     uint32 numFailures;
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
pub struct PeerAddress {
    pub ip: PeerAddressIp,
    pub port: u32,
    pub num_failures: u32,
}

impl ReadXdr for PeerAddress {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ip: PeerAddressIp::read_xdr(r)?,
                port: u32::read_xdr(r)?,
                num_failures: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for PeerAddress {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ip.write_xdr(w)?;
            self.port.write_xdr(w)?;
            self.num_failures.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`PeerAddress`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyPeerAddress(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyPeerAddress {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyPeerAddress {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.ip().cmp(&other.ip()))
            .then_with(|| self.port().cmp(&other.port()))
            .then_with(|| self.num_failures().cmp(&other.num_failures()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyPeerAddress {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyPeerAddressIp as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(8).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyPeerAddressIp as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
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
impl From<LazyHandle> for LazyPeerAddress {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyPeerAddress {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyPeerAddress {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyPeerAddress {
    /// Access field `ip`.
    #[must_use]
    pub fn ip(&self) -> LazyPeerAddressIp {
        <LazyPeerAddressIp as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `port`.
    #[must_use]
    pub fn port(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyPeerAddressIp as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `num_failures`.
    #[must_use]
    pub fn num_failures(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyPeerAddressIp as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&PeerAddress> for LazyPeerAddress {
    type Error = Error;
    fn try_from(val: &PeerAddress) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyPeerAddress> for PeerAddress {
    type Error = Error;
    fn try_from(lazy: &LazyPeerAddress) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
