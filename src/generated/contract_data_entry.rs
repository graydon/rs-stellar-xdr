#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ContractDataEntry is an XDR Struct defined as:
///
/// ```text
/// struct ContractDataEntry {
///     ExtensionPoint ext;
///
///     SCAddress contract;
///     SCVal key;
///     ContractDataDurability durability;
///     SCVal val;
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
pub struct ContractDataEntry {
    pub ext: ExtensionPoint,
    pub contract: ScAddress,
    pub key: ScVal,
    pub durability: ContractDataDurability,
    pub val: ScVal,
}

impl ReadXdr for ContractDataEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ExtensionPoint::read_xdr(r)?,
                contract: ScAddress::read_xdr(r)?,
                key: ScVal::read_xdr(r)?,
                durability: ContractDataDurability::read_xdr(r)?,
                val: ScVal::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ContractDataEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.contract.write_xdr(w)?;
            self.key.write_xdr(w)?;
            self.durability.write_xdr(w)?;
            self.val.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ContractDataEntry`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyContractDataEntry(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyContractDataEntry {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyContractDataEntry {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.ext().cmp(&other.ext()))
            .then_with(|| self.contract().cmp(&other.contract()))
            .then_with(|| self.key().cmp(&other.key()))
            .then_with(|| self.durability().cmp(&other.durability()))
            .then_with(|| self.val().cmp(&other.val()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyContractDataEntry {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyExtensionPoint as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyScAddress as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyScVal as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(4).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <ContractDataDurability as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        {
            let field_len = <LazyScVal as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyScAddress as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyScVal as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        pos += <LazyScVal as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos
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
impl From<LazyHandle> for LazyContractDataEntry {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyContractDataEntry {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyContractDataEntry {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyContractDataEntry {
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazyExtensionPoint {
        <LazyExtensionPoint as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `contract`.
    #[must_use]
    pub fn contract(&self) -> LazyScAddress {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyScAddress as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `key`.
    #[must_use]
    pub fn key(&self) -> LazyScVal {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyScAddress as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyScVal as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `durability`.
    #[must_use]
    pub fn durability(&self) -> ContractDataDurability {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyScAddress as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyScVal as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <ContractDataDurability as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `val`.
    #[must_use]
    pub fn val(&self) -> LazyScVal {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyScAddress as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyScVal as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        <LazyScVal as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ContractDataEntry> for LazyContractDataEntry {
    type Error = Error;
    fn try_from(val: &ContractDataEntry) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyContractDataEntry> for ContractDataEntry {
    type Error = Error;
    fn try_from(lazy: &LazyContractDataEntry) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
