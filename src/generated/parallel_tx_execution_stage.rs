#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ParallelTxExecutionStage is an XDR Typedef defined as:
///
/// ```text
/// typedef DependentTxCluster ParallelTxExecutionStage<>;
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
pub struct ParallelTxExecutionStage(pub VecM<DependentTxCluster>);

impl From<ParallelTxExecutionStage> for VecM<DependentTxCluster> {
    #[must_use]
    fn from(x: ParallelTxExecutionStage) -> Self {
        x.0
    }
}

impl From<VecM<DependentTxCluster>> for ParallelTxExecutionStage {
    #[must_use]
    fn from(x: VecM<DependentTxCluster>) -> Self {
        ParallelTxExecutionStage(x)
    }
}

impl AsRef<VecM<DependentTxCluster>> for ParallelTxExecutionStage {
    #[must_use]
    fn as_ref(&self) -> &VecM<DependentTxCluster> {
        &self.0
    }
}

impl ReadXdr for ParallelTxExecutionStage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = VecM::<DependentTxCluster>::read_xdr(r)?;
            let v = ParallelTxExecutionStage(i);
            Ok(v)
        })
    }
}

impl WriteXdr for ParallelTxExecutionStage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for ParallelTxExecutionStage {
    type Target = VecM<DependentTxCluster>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ParallelTxExecutionStage> for Vec<DependentTxCluster> {
    #[must_use]
    fn from(x: ParallelTxExecutionStage) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<DependentTxCluster>> for ParallelTxExecutionStage {
    type Error = Error;
    fn try_from(x: Vec<DependentTxCluster>) -> Result<Self, Error> {
        Ok(ParallelTxExecutionStage(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<DependentTxCluster>> for ParallelTxExecutionStage {
    type Error = Error;
    fn try_from(x: &Vec<DependentTxCluster>) -> Result<Self, Error> {
        Ok(ParallelTxExecutionStage(x.try_into()?))
    }
}

impl AsRef<Vec<DependentTxCluster>> for ParallelTxExecutionStage {
    #[must_use]
    fn as_ref(&self) -> &Vec<DependentTxCluster> {
        &self.0 .0
    }
}

impl AsRef<[DependentTxCluster]> for ParallelTxExecutionStage {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[DependentTxCluster] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[DependentTxCluster] {
        self.0 .0
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ParallelTxExecutionStage`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyParallelTxExecutionStage(LazyVecM<LazyDependentTxCluster>);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyParallelTxExecutionStage {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyParallelTxExecutionStage {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyParallelTxExecutionStage {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        <LazyVecM<LazyDependentTxCluster> as LazyXdr>::xdr_validate(buf, depth)
    }

    #[inline]
    fn xdr_len(buf: &[u8]) -> u32 {
        <LazyVecM<LazyDependentTxCluster> as LazyXdr>::xdr_len(buf)
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        Self(<LazyVecM<LazyDependentTxCluster> as LazyXdr>::from_xdr_at(
            parent, offset,
        ))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyParallelTxExecutionStage {
    fn from(h: LazyHandle) -> Self {
        Self(<LazyVecM<LazyDependentTxCluster>>::from(h))
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyParallelTxExecutionStage {
    fn as_ref(&self) -> &LazyHandle {
        self.0.as_ref()
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyParallelTxExecutionStage {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(<LazyVecM<LazyDependentTxCluster>>::from(
            LazyHandle::from_arc(buf, 0, len),
        )))
    }
}
#[cfg(feature = "alloc")]
impl core::ops::Deref for LazyParallelTxExecutionStage {
    type Target = LazyVecM<LazyDependentTxCluster>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ParallelTxExecutionStage> for LazyParallelTxExecutionStage {
    type Error = Error;
    fn try_from(val: &ParallelTxExecutionStage) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyParallelTxExecutionStage> for ParallelTxExecutionStage {
    type Error = Error;
    fn try_from(lazy: &LazyParallelTxExecutionStage) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
