#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// DependentTxCluster is an XDR Typedef defined as:
///
/// ```text
/// typedef TransactionEnvelope DependentTxCluster<>;
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
pub struct DependentTxCluster(pub VecM<TransactionEnvelope>);

impl From<DependentTxCluster> for VecM<TransactionEnvelope> {
    #[must_use]
    fn from(x: DependentTxCluster) -> Self {
        x.0
    }
}

impl From<VecM<TransactionEnvelope>> for DependentTxCluster {
    #[must_use]
    fn from(x: VecM<TransactionEnvelope>) -> Self {
        DependentTxCluster(x)
    }
}

impl AsRef<VecM<TransactionEnvelope>> for DependentTxCluster {
    #[must_use]
    fn as_ref(&self) -> &VecM<TransactionEnvelope> {
        &self.0
    }
}

impl ReadXdr for DependentTxCluster {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = VecM::<TransactionEnvelope>::read_xdr(r)?;
            let v = DependentTxCluster(i);
            Ok(v)
        })
    }
}

impl WriteXdr for DependentTxCluster {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for DependentTxCluster {
    type Target = VecM<TransactionEnvelope>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DependentTxCluster> for Vec<TransactionEnvelope> {
    #[must_use]
    fn from(x: DependentTxCluster) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<TransactionEnvelope>> for DependentTxCluster {
    type Error = Error;
    fn try_from(x: Vec<TransactionEnvelope>) -> Result<Self, Error> {
        Ok(DependentTxCluster(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<TransactionEnvelope>> for DependentTxCluster {
    type Error = Error;
    fn try_from(x: &Vec<TransactionEnvelope>) -> Result<Self, Error> {
        Ok(DependentTxCluster(x.try_into()?))
    }
}

impl AsRef<Vec<TransactionEnvelope>> for DependentTxCluster {
    #[must_use]
    fn as_ref(&self) -> &Vec<TransactionEnvelope> {
        &self.0 .0
    }
}

impl AsRef<[TransactionEnvelope]> for DependentTxCluster {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[TransactionEnvelope] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[TransactionEnvelope] {
        self.0 .0
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`DependentTxCluster`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyDependentTxCluster(LazyVecM<LazyTransactionEnvelope>);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyDependentTxCluster {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyDependentTxCluster {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyDependentTxCluster {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        <LazyVecM<LazyTransactionEnvelope> as LazyXdr>::xdr_validate(buf, depth)
    }

    #[inline]
    fn xdr_len(buf: &[u8]) -> u32 {
        <LazyVecM<LazyTransactionEnvelope> as LazyXdr>::xdr_len(buf)
    }

    fn from_xdr_consume(parent: &LazyHandle, buf: &mut &[u8]) -> Self {
        Self(<LazyVecM<LazyTransactionEnvelope> as LazyXdr>::from_xdr_consume(parent, buf))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyDependentTxCluster {
    fn from(h: LazyHandle) -> Self {
        Self(<LazyVecM<LazyTransactionEnvelope>>::from(h))
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyDependentTxCluster {
    fn as_ref(&self) -> &LazyHandle {
        self.0.as_ref()
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyDependentTxCluster {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(<LazyVecM<LazyTransactionEnvelope>>::from(
            LazyHandle::from_arc(buf, 0, len),
        )))
    }
}
#[cfg(feature = "alloc")]
impl core::ops::Deref for LazyDependentTxCluster {
    type Target = LazyVecM<LazyTransactionEnvelope>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&DependentTxCluster> for LazyDependentTxCluster {
    type Error = Error;
    fn try_from(val: &DependentTxCluster) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyDependentTxCluster> for DependentTxCluster {
    type Error = Error;
    fn try_from(lazy: &LazyDependentTxCluster) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
