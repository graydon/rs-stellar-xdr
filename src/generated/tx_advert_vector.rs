#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// TxAdvertVector is an XDR Typedef defined as:
///
/// ```text
/// typedef Hash TxAdvertVector<TX_ADVERT_VECTOR_MAX_SIZE>;
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
pub struct TxAdvertVector(pub VecM<Hash, 1000>);

impl From<TxAdvertVector> for VecM<Hash, 1000> {
    #[must_use]
    fn from(x: TxAdvertVector) -> Self {
        x.0
    }
}

impl From<VecM<Hash, 1000>> for TxAdvertVector {
    #[must_use]
    fn from(x: VecM<Hash, 1000>) -> Self {
        TxAdvertVector(x)
    }
}

impl AsRef<VecM<Hash, 1000>> for TxAdvertVector {
    #[must_use]
    fn as_ref(&self) -> &VecM<Hash, 1000> {
        &self.0
    }
}

impl ReadXdr for TxAdvertVector {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = VecM::<Hash, 1000>::read_xdr(r)?;
            let v = TxAdvertVector(i);
            Ok(v)
        })
    }
}

impl WriteXdr for TxAdvertVector {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for TxAdvertVector {
    type Target = VecM<Hash, 1000>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TxAdvertVector> for Vec<Hash> {
    #[must_use]
    fn from(x: TxAdvertVector) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<Hash>> for TxAdvertVector {
    type Error = Error;
    fn try_from(x: Vec<Hash>) -> Result<Self, Error> {
        Ok(TxAdvertVector(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<Hash>> for TxAdvertVector {
    type Error = Error;
    fn try_from(x: &Vec<Hash>) -> Result<Self, Error> {
        Ok(TxAdvertVector(x.try_into()?))
    }
}

impl AsRef<Vec<Hash>> for TxAdvertVector {
    #[must_use]
    fn as_ref(&self) -> &Vec<Hash> {
        &self.0 .0
    }
}

impl AsRef<[Hash]> for TxAdvertVector {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[Hash] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[Hash] {
        self.0 .0
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`TxAdvertVector`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyTxAdvertVector(LazyVecM<LazyHash, 1000>);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyTxAdvertVector {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyTxAdvertVector {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyTxAdvertVector {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        <LazyVecM<LazyHash, 1000> as LazyXdr>::xdr_validate(buf, depth)
    }

    #[inline]
    fn xdr_len(buf: &[u8]) -> u32 {
        <LazyVecM<LazyHash, 1000> as LazyXdr>::xdr_len(buf)
    }

    fn from_xdr_consume(parent: &LazyHandle, buf: &mut &[u8]) -> Self {
        Self(<LazyVecM<LazyHash, 1000> as LazyXdr>::from_xdr_consume(
            parent, buf,
        ))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyTxAdvertVector {
    fn from(h: LazyHandle) -> Self {
        Self(<LazyVecM<LazyHash, 1000>>::from(h))
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyTxAdvertVector {
    fn as_ref(&self) -> &LazyHandle {
        self.0.as_ref()
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyTxAdvertVector {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(<LazyVecM<LazyHash, 1000>>::from(
            LazyHandle::from_arc(buf, 0, len),
        )))
    }
}
#[cfg(feature = "alloc")]
impl core::ops::Deref for LazyTxAdvertVector {
    type Target = LazyVecM<LazyHash, 1000>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&TxAdvertVector> for LazyTxAdvertVector {
    type Error = Error;
    fn try_from(val: &TxAdvertVector) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyTxAdvertVector> for TxAdvertVector {
    type Error = Error;
    fn try_from(lazy: &LazyTxAdvertVector) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
