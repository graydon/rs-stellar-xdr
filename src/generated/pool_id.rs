#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// PoolId is an XDR Typedef defined as:
///
/// ```text
/// typedef Hash PoolID;
/// ```
///
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
#[derive(Debug)]
pub struct PoolId(pub Hash);

impl From<PoolId> for Hash {
    #[must_use]
    fn from(x: PoolId) -> Self {
        x.0
    }
}

impl From<Hash> for PoolId {
    #[must_use]
    fn from(x: Hash) -> Self {
        PoolId(x)
    }
}

impl AsRef<Hash> for PoolId {
    #[must_use]
    fn as_ref(&self) -> &Hash {
        &self.0
    }
}

impl ReadXdr for PoolId {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = Hash::read_xdr(r)?;
            let v = PoolId(i);
            Ok(v)
        })
    }
}

impl WriteXdr for PoolId {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`PoolId`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyPoolId(LazyHash);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyPoolId {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyPoolId {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyPoolId {
    const FIXED_XDR_SIZE: Option<u32> = Some(32);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        <LazyHash as LazyXdr>::xdr_validate(buf, depth)
    }

    #[inline]
    fn xdr_len(buf: &[u8]) -> u32 {
        <LazyHash as LazyXdr>::xdr_len(buf)
    }

    fn from_xdr_consume(parent: &LazyHandle, buf: &mut &[u8]) -> Self {
        Self(<LazyHash as LazyXdr>::from_xdr_consume(parent, buf))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyPoolId {
    fn from(h: LazyHandle) -> Self {
        Self(<LazyHash>::from(h))
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyPoolId {
    fn as_ref(&self) -> &LazyHandle {
        self.0.as_ref()
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyPoolId {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(<LazyHash>::from(LazyHandle::from_arc(buf, 0, len))))
    }
}
#[cfg(feature = "alloc")]
impl core::ops::Deref for LazyPoolId {
    type Target = LazyHash;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&PoolId> for LazyPoolId {
    type Error = Error;
    fn try_from(val: &PoolId) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyPoolId> for PoolId {
    type Error = Error;
    fn try_from(lazy: &LazyPoolId) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
