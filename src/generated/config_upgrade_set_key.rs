#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ConfigUpgradeSetKey is an XDR Struct defined as:
///
/// ```text
/// struct ConfigUpgradeSetKey {
///     ContractID contractID;
///     Hash contentHash;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ConfigUpgradeSetKey {
    pub contract_id: ContractId,
    pub content_hash: Hash,
}

impl ReadXdr for ConfigUpgradeSetKey {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                contract_id: ContractId::read_xdr(r)?,
                content_hash: Hash::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ConfigUpgradeSetKey {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.contract_id.write_xdr(w)?;
            self.content_hash.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ConfigUpgradeSetKey`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyConfigUpgradeSetKey(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyConfigUpgradeSetKey {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyConfigUpgradeSetKey {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.contract_id().cmp(&other.contract_id()))
            .then_with(|| self.content_hash().cmp(&other.content_hash()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyConfigUpgradeSetKey {
    const FIXED_XDR_SIZE: Option<u32> = Some(64);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(64).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyContractId as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        <LazyHash as LazyXdr>::xdr_validate(&buf[(pos + 32) as usize..], depth)?;
        pos = next_pos;
        Ok(pos)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        64
    }

    fn from_xdr_consume(parent: &LazyHandle, buf: &mut &[u8]) -> Self {
        let len = Self::xdr_len(buf);
        let offset = (parent.len() as usize - buf.len()) as u32;
        let handle = parent.sub_handle(offset, len);
        *buf = &buf[len as usize..];
        Self(handle)
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyConfigUpgradeSetKey {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyConfigUpgradeSetKey {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyConfigUpgradeSetKey {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyConfigUpgradeSetKey {
    /// Access field `contract_id`.
    #[must_use]
    pub fn contract_id(&self) -> LazyContractId {
        <LazyContractId as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `content_hash`.
    #[must_use]
    pub fn content_hash(&self) -> LazyHash {
        <LazyHash as LazyXdr>::from_xdr_at(&self.0, 32)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ConfigUpgradeSetKey> for LazyConfigUpgradeSetKey {
    type Error = Error;
    fn try_from(val: &ConfigUpgradeSetKey) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyConfigUpgradeSetKey> for ConfigUpgradeSetKey {
    type Error = Error;
    fn try_from(lazy: &LazyConfigUpgradeSetKey) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
