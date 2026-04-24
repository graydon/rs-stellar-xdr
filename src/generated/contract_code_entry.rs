#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ContractCodeEntry is an XDR Struct defined as:
///
/// ```text
/// struct ContractCodeEntry {
///     union switch (int v)
///     {
///         case 0:
///             void;
///         case 1:
///             struct
///             {
///                 ExtensionPoint ext;
///                 ContractCodeCostInputs costInputs;
///             } v1;
///     } ext;
///
///     Hash hash;
///     opaque code<>;
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
pub struct ContractCodeEntry {
    pub ext: ContractCodeEntryExt,
    pub hash: Hash,
    pub code: BytesM,
}

impl ReadXdr for ContractCodeEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ContractCodeEntryExt::read_xdr(r)?,
                hash: Hash::read_xdr(r)?,
                code: BytesM::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ContractCodeEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.hash.write_xdr(w)?;
            self.code.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ContractCodeEntry`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyContractCodeEntry(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyContractCodeEntry {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyContractCodeEntry {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.ext().cmp(&other.ext()))
            .then_with(|| self.hash().cmp(&other.hash()))
            .then_with(|| self.code().cmp(&other.code()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyContractCodeEntry {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyContractCodeEntryExt as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(32).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyHash as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        {
            let field_len = <LazyBytesM as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyContractCodeEntryExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 32;
        pos += <LazyBytesM as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyContractCodeEntry {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyContractCodeEntry {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyContractCodeEntry {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyContractCodeEntry {
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazyContractCodeEntryExt {
        <LazyContractCodeEntryExt as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `hash`.
    #[must_use]
    pub fn hash(&self) -> LazyHash {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyContractCodeEntryExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyHash as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `code`.
    #[must_use]
    pub fn code(&self) -> LazyBytesM {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyContractCodeEntryExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 32;
        <LazyBytesM as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ContractCodeEntry> for LazyContractCodeEntry {
    type Error = Error;
    fn try_from(val: &ContractCodeEntry) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyContractCodeEntry> for ContractCodeEntry {
    type Error = Error;
    fn try_from(lazy: &LazyContractCodeEntry) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
