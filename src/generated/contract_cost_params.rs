#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ContractCostParams is an XDR Typedef defined as:
///
/// ```text
/// typedef ContractCostParamEntry ContractCostParams<CONTRACT_COST_COUNT_LIMIT>;
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
pub struct ContractCostParams(pub VecM<ContractCostParamEntry, 1024>);

impl From<ContractCostParams> for VecM<ContractCostParamEntry, 1024> {
    #[must_use]
    fn from(x: ContractCostParams) -> Self {
        x.0
    }
}

impl From<VecM<ContractCostParamEntry, 1024>> for ContractCostParams {
    #[must_use]
    fn from(x: VecM<ContractCostParamEntry, 1024>) -> Self {
        ContractCostParams(x)
    }
}

impl AsRef<VecM<ContractCostParamEntry, 1024>> for ContractCostParams {
    #[must_use]
    fn as_ref(&self) -> &VecM<ContractCostParamEntry, 1024> {
        &self.0
    }
}

impl ReadXdr for ContractCostParams {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = VecM::<ContractCostParamEntry, 1024>::read_xdr(r)?;
            let v = ContractCostParams(i);
            Ok(v)
        })
    }
}

impl WriteXdr for ContractCostParams {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for ContractCostParams {
    type Target = VecM<ContractCostParamEntry, 1024>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ContractCostParams> for Vec<ContractCostParamEntry> {
    #[must_use]
    fn from(x: ContractCostParams) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<ContractCostParamEntry>> for ContractCostParams {
    type Error = Error;
    fn try_from(x: Vec<ContractCostParamEntry>) -> Result<Self, Error> {
        Ok(ContractCostParams(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<ContractCostParamEntry>> for ContractCostParams {
    type Error = Error;
    fn try_from(x: &Vec<ContractCostParamEntry>) -> Result<Self, Error> {
        Ok(ContractCostParams(x.try_into()?))
    }
}

impl AsRef<Vec<ContractCostParamEntry>> for ContractCostParams {
    #[must_use]
    fn as_ref(&self) -> &Vec<ContractCostParamEntry> {
        &self.0 .0
    }
}

impl AsRef<[ContractCostParamEntry]> for ContractCostParams {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[ContractCostParamEntry] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[ContractCostParamEntry] {
        self.0 .0
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ContractCostParams`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyContractCostParams(LazyVecM<LazyContractCostParamEntry, 1024>);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyContractCostParams {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyContractCostParams {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyContractCostParams {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        <LazyVecM<LazyContractCostParamEntry, 1024> as LazyXdr>::xdr_validate(buf, depth)
    }

    #[inline]
    fn xdr_len(buf: &[u8]) -> u32 {
        <LazyVecM<LazyContractCostParamEntry, 1024> as LazyXdr>::xdr_len(buf)
    }

    fn from_xdr_consume(parent: &LazyHandle, buf: &mut &[u8]) -> Self {
        Self(<LazyVecM<LazyContractCostParamEntry, 1024> as LazyXdr>::from_xdr_consume(parent, buf))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyContractCostParams {
    fn from(h: LazyHandle) -> Self {
        Self(<LazyVecM<LazyContractCostParamEntry, 1024>>::from(h))
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyContractCostParams {
    fn as_ref(&self) -> &LazyHandle {
        self.0.as_ref()
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyContractCostParams {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(<LazyVecM<LazyContractCostParamEntry, 1024>>::from(
            LazyHandle::from_arc(buf, 0, len),
        )))
    }
}
#[cfg(feature = "alloc")]
impl core::ops::Deref for LazyContractCostParams {
    type Target = LazyVecM<LazyContractCostParamEntry, 1024>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ContractCostParams> for LazyContractCostParams {
    type Error = Error;
    fn try_from(val: &ContractCostParams) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyContractCostParams> for ContractCostParams {
    type Error = Error;
    fn try_from(lazy: &LazyContractCostParams) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
