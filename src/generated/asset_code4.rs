#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// AssetCode4 is an XDR Typedef defined as:
///
/// ```text
/// typedef opaque AssetCode4[4];
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
pub struct AssetCode4(pub [u8; 4]);

impl core::fmt::Debug for AssetCode4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let v = &self.0;
        write!(f, "AssetCode4(")?;
        for b in v {
            write!(f, "{b:02x}")?;
        }
        write!(f, ")")?;
        Ok(())
    }
}
impl From<AssetCode4> for [u8; 4] {
    #[must_use]
    fn from(x: AssetCode4) -> Self {
        x.0
    }
}

impl From<[u8; 4]> for AssetCode4 {
    #[must_use]
    fn from(x: [u8; 4]) -> Self {
        AssetCode4(x)
    }
}

impl AsRef<[u8; 4]> for AssetCode4 {
    #[must_use]
    fn as_ref(&self) -> &[u8; 4] {
        &self.0
    }
}

impl ReadXdr for AssetCode4 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = <[u8; 4]>::read_xdr(r)?;
            let v = AssetCode4(i);
            Ok(v)
        })
    }
}

impl WriteXdr for AssetCode4 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl AssetCode4 {
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<Vec<u8>> for AssetCode4 {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self, Error> {
        x.as_slice().try_into()
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for AssetCode4 {
    type Error = Error;
    fn try_from(x: &Vec<u8>) -> Result<Self, Error> {
        x.as_slice().try_into()
    }
}

impl TryFrom<&[u8]> for AssetCode4 {
    type Error = Error;
    fn try_from(x: &[u8]) -> Result<Self, Error> {
        Ok(AssetCode4(x.try_into()?))
    }
}

impl AsRef<[u8]> for AssetCode4 {
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`AssetCode4`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyAssetCode4(LazyOpaqueFixed<4>);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyAssetCode4 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyAssetCode4 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyAssetCode4 {
    const FIXED_XDR_SIZE: Option<u32> = Some(4);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        <LazyOpaqueFixed<4> as LazyXdr>::xdr_validate(buf, depth)
    }

    #[inline]
    fn xdr_len(buf: &[u8]) -> u32 {
        <LazyOpaqueFixed<4> as LazyXdr>::xdr_len(buf)
    }

    fn from_xdr_consume(parent: &LazyHandle, buf: &mut &[u8]) -> Self {
        Self(<LazyOpaqueFixed<4> as LazyXdr>::from_xdr_consume(
            parent, buf,
        ))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyAssetCode4 {
    fn from(h: LazyHandle) -> Self {
        Self(<LazyOpaqueFixed<4>>::from(h))
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyAssetCode4 {
    fn as_ref(&self) -> &LazyHandle {
        self.0.as_ref()
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyAssetCode4 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(<LazyOpaqueFixed<4>>::from(LazyHandle::from_arc(
            buf, 0, len,
        ))))
    }
}
#[cfg(feature = "alloc")]
impl core::ops::Deref for LazyAssetCode4 {
    type Target = LazyOpaqueFixed<4>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&AssetCode4> for LazyAssetCode4 {
    type Error = Error;
    fn try_from(val: &AssetCode4) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyAssetCode4> for AssetCode4 {
    type Error = Error;
    fn try_from(lazy: &LazyAssetCode4) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
