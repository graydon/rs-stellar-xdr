#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SorobanAuthorizationEntries is an XDR Typedef defined as:
///
/// ```text
/// typedef SorobanAuthorizationEntry SorobanAuthorizationEntries<>;
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
pub struct SorobanAuthorizationEntries(pub VecM<SorobanAuthorizationEntry>);

impl From<SorobanAuthorizationEntries> for VecM<SorobanAuthorizationEntry> {
    #[must_use]
    fn from(x: SorobanAuthorizationEntries) -> Self {
        x.0
    }
}

impl From<VecM<SorobanAuthorizationEntry>> for SorobanAuthorizationEntries {
    #[must_use]
    fn from(x: VecM<SorobanAuthorizationEntry>) -> Self {
        SorobanAuthorizationEntries(x)
    }
}

impl AsRef<VecM<SorobanAuthorizationEntry>> for SorobanAuthorizationEntries {
    #[must_use]
    fn as_ref(&self) -> &VecM<SorobanAuthorizationEntry> {
        &self.0
    }
}

impl ReadXdr for SorobanAuthorizationEntries {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = VecM::<SorobanAuthorizationEntry>::read_xdr(r)?;
            let v = SorobanAuthorizationEntries(i);
            Ok(v)
        })
    }
}

impl WriteXdr for SorobanAuthorizationEntries {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for SorobanAuthorizationEntries {
    type Target = VecM<SorobanAuthorizationEntry>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<SorobanAuthorizationEntries> for Vec<SorobanAuthorizationEntry> {
    #[must_use]
    fn from(x: SorobanAuthorizationEntries) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<SorobanAuthorizationEntry>> for SorobanAuthorizationEntries {
    type Error = Error;
    fn try_from(x: Vec<SorobanAuthorizationEntry>) -> Result<Self, Error> {
        Ok(SorobanAuthorizationEntries(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<SorobanAuthorizationEntry>> for SorobanAuthorizationEntries {
    type Error = Error;
    fn try_from(x: &Vec<SorobanAuthorizationEntry>) -> Result<Self, Error> {
        Ok(SorobanAuthorizationEntries(x.try_into()?))
    }
}

impl AsRef<Vec<SorobanAuthorizationEntry>> for SorobanAuthorizationEntries {
    #[must_use]
    fn as_ref(&self) -> &Vec<SorobanAuthorizationEntry> {
        &self.0 .0
    }
}

impl AsRef<[SorobanAuthorizationEntry]> for SorobanAuthorizationEntries {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[SorobanAuthorizationEntry] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[SorobanAuthorizationEntry] {
        self.0 .0
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`SorobanAuthorizationEntries`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySorobanAuthorizationEntries(LazyVecM<LazySorobanAuthorizationEntry>);
#[cfg(feature = "alloc")]
impl PartialOrd for LazySorobanAuthorizationEntries {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazySorobanAuthorizationEntries {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazySorobanAuthorizationEntries {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        <LazyVecM<LazySorobanAuthorizationEntry> as LazyXdr>::xdr_validate(buf, depth)
    }

    #[inline]
    fn xdr_len(buf: &[u8]) -> u32 {
        <LazyVecM<LazySorobanAuthorizationEntry> as LazyXdr>::xdr_len(buf)
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        Self(<LazyVecM<LazySorobanAuthorizationEntry> as LazyXdr>::from_xdr_at(parent, offset))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazySorobanAuthorizationEntries {
    fn from(h: LazyHandle) -> Self {
        Self(<LazyVecM<LazySorobanAuthorizationEntry>>::from(h))
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazySorobanAuthorizationEntries {
    fn as_ref(&self) -> &LazyHandle {
        self.0.as_ref()
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazySorobanAuthorizationEntries {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(<LazyVecM<LazySorobanAuthorizationEntry>>::from(
            LazyHandle::from_arc(buf, 0, len),
        )))
    }
}
#[cfg(feature = "alloc")]
impl core::ops::Deref for LazySorobanAuthorizationEntries {
    type Target = LazyVecM<LazySorobanAuthorizationEntry>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&SorobanAuthorizationEntries> for LazySorobanAuthorizationEntries {
    type Error = Error;
    fn try_from(val: &SorobanAuthorizationEntries) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazySorobanAuthorizationEntries> for SorobanAuthorizationEntries {
    type Error = Error;
    fn try_from(lazy: &LazySorobanAuthorizationEntries) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
