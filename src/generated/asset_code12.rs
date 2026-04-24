#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// AssetCode12 is an XDR Typedef defined as:
///
/// ```text
/// typedef opaque AssetCode12[12];
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
pub struct AssetCode12(pub [u8; 12]);

impl core::fmt::Debug for AssetCode12 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let v = &self.0;
        write!(f, "AssetCode12(")?;
        for b in v {
            write!(f, "{b:02x}")?;
        }
        write!(f, ")")?;
        Ok(())
    }
}
impl From<AssetCode12> for [u8; 12] {
    #[must_use]
    fn from(x: AssetCode12) -> Self {
        x.0
    }
}

impl From<[u8; 12]> for AssetCode12 {
    #[must_use]
    fn from(x: [u8; 12]) -> Self {
        AssetCode12(x)
    }
}

impl AsRef<[u8; 12]> for AssetCode12 {
    #[must_use]
    fn as_ref(&self) -> &[u8; 12] {
        &self.0
    }
}

impl ReadXdr for AssetCode12 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = <[u8; 12]>::read_xdr(r)?;
            let v = AssetCode12(i);
            Ok(v)
        })
    }
}

impl WriteXdr for AssetCode12 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl AssetCode12 {
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<Vec<u8>> for AssetCode12 {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self, Error> {
        x.as_slice().try_into()
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for AssetCode12 {
    type Error = Error;
    fn try_from(x: &Vec<u8>) -> Result<Self, Error> {
        x.as_slice().try_into()
    }
}

impl TryFrom<&[u8]> for AssetCode12 {
    type Error = Error;
    fn try_from(x: &[u8]) -> Result<Self, Error> {
        Ok(AssetCode12(x.try_into()?))
    }
}

impl AsRef<[u8]> for AssetCode12 {
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`AssetCode12`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyAssetCode12(LazyOpaqueFixed<12>);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyAssetCode12 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyAssetCode12 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyAssetCode12 {
    const FIXED_XDR_SIZE: Option<u32> = Some(12);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        <LazyOpaqueFixed<12> as LazyXdr>::xdr_validate(buf, depth)
    }

    #[inline]
    fn xdr_len(buf: &[u8]) -> u32 {
        <LazyOpaqueFixed<12> as LazyXdr>::xdr_len(buf)
    }

    fn from_xdr_consume(parent: &LazyHandle, buf: &mut &[u8]) -> Self {
        Self(<LazyOpaqueFixed<12> as LazyXdr>::from_xdr_consume(
            parent, buf,
        ))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyAssetCode12 {
    fn from(h: LazyHandle) -> Self {
        Self(<LazyOpaqueFixed<12>>::from(h))
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyAssetCode12 {
    fn as_ref(&self) -> &LazyHandle {
        self.0.as_ref()
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyAssetCode12 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(<LazyOpaqueFixed<12>>::from(LazyHandle::from_arc(
            buf, 0, len,
        ))))
    }
}
#[cfg(feature = "alloc")]
impl core::ops::Deref for LazyAssetCode12 {
    type Target = LazyOpaqueFixed<12>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&AssetCode12> for LazyAssetCode12 {
    type Error = Error;
    fn try_from(val: &AssetCode12) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyAssetCode12> for AssetCode12 {
    type Error = Error;
    fn try_from(lazy: &LazyAssetCode12) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
