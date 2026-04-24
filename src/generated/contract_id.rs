#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ContractId is an XDR Typedef defined as:
///
/// ```text
/// typedef Hash ContractID;
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
pub struct ContractId(pub Hash);

impl From<ContractId> for Hash {
    #[must_use]
    fn from(x: ContractId) -> Self {
        x.0
    }
}

impl From<Hash> for ContractId {
    #[must_use]
    fn from(x: Hash) -> Self {
        ContractId(x)
    }
}

impl AsRef<Hash> for ContractId {
    #[must_use]
    fn as_ref(&self) -> &Hash {
        &self.0
    }
}

impl ReadXdr for ContractId {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = Hash::read_xdr(r)?;
            let v = ContractId(i);
            Ok(v)
        })
    }
}

impl WriteXdr for ContractId {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ContractId`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyContractId(LazyHash);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyContractId {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyContractId {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyContractId {
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
impl From<LazyHandle> for LazyContractId {
    fn from(h: LazyHandle) -> Self {
        Self(<LazyHash>::from(h))
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyContractId {
    fn as_ref(&self) -> &LazyHandle {
        self.0.as_ref()
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyContractId {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(<LazyHash>::from(LazyHandle::from_arc(buf, 0, len))))
    }
}
#[cfg(feature = "alloc")]
impl core::ops::Deref for LazyContractId {
    type Target = LazyHash;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ContractId> for LazyContractId {
    type Error = Error;
    fn try_from(val: &ContractId) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyContractId> for ContractId {
    type Error = Error;
    fn try_from(lazy: &LazyContractId) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
