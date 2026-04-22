#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// UpgradeType is an XDR Typedef defined as:
///
/// ```text
/// typedef opaque UpgradeType<128>;
/// ```
///
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug)]
pub struct UpgradeType(pub BytesM<128>);

impl From<UpgradeType> for BytesM<128> {
    #[must_use]
    fn from(x: UpgradeType) -> Self {
        x.0
    }
}

impl From<BytesM<128>> for UpgradeType {
    #[must_use]
    fn from(x: BytesM<128>) -> Self {
        UpgradeType(x)
    }
}

impl AsRef<BytesM<128>> for UpgradeType {
    #[must_use]
    fn as_ref(&self) -> &BytesM<128> {
        &self.0
    }
}

impl ReadXdr for UpgradeType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = BytesM::<128>::read_xdr(r)?;
            let v = UpgradeType(i);
            Ok(v)
        })
    }
}

impl WriteXdr for UpgradeType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for UpgradeType {
    type Target = BytesM<128>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<UpgradeType> for Vec<u8> {
    #[must_use]
    fn from(x: UpgradeType) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<u8>> for UpgradeType {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self, Error> {
        Ok(UpgradeType(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for UpgradeType {
    type Error = Error;
    fn try_from(x: &Vec<u8>) -> Result<Self, Error> {
        Ok(UpgradeType(x.try_into()?))
    }
}

impl AsRef<Vec<u8>> for UpgradeType {
    #[must_use]
    fn as_ref(&self) -> &Vec<u8> {
        &self.0 .0
    }
}

impl AsRef<[u8]> for UpgradeType {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        self.0 .0
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`UpgradeType`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyUpgradeType(LazyBytesM<128>);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyUpgradeType {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyUpgradeType {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyUpgradeType {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        <LazyBytesM<128> as LazyXdr>::xdr_validate(buf, depth)
    }

    #[inline]
    fn xdr_len(buf: &[u8]) -> u32 {
        <LazyBytesM<128> as LazyXdr>::xdr_len(buf)
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        Self(<LazyBytesM<128> as LazyXdr>::from_xdr_at(parent, offset))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyUpgradeType {
    fn from(h: LazyHandle) -> Self {
        Self(<LazyBytesM<128>>::from(h))
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyUpgradeType {
    fn as_ref(&self) -> &LazyHandle {
        self.0.as_ref()
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyUpgradeType {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(<LazyBytesM<128>>::from(LazyHandle::from_arc(
            buf, 0, len,
        ))))
    }
}
#[cfg(feature = "alloc")]
impl core::ops::Deref for LazyUpgradeType {
    type Target = LazyBytesM<128>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&UpgradeType> for LazyUpgradeType {
    type Error = Error;
    fn try_from(val: &UpgradeType) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyUpgradeType> for UpgradeType {
    type Error = Error;
    fn try_from(lazy: &LazyUpgradeType) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
