#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ContractCostParamEntry is an XDR Struct defined as:
///
/// ```text
/// struct ContractCostParamEntry {
///     // use `ext` to add more terms (e.g. higher order polynomials) in the future
///     ExtensionPoint ext;
///
///     int64 constTerm;
///     int64 linearTerm;
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
pub struct ContractCostParamEntry {
    pub ext: ExtensionPoint,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub const_term: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub linear_term: i64,
}

impl ReadXdr for ContractCostParamEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ExtensionPoint::read_xdr(r)?,
                const_term: i64::read_xdr(r)?,
                linear_term: i64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ContractCostParamEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.const_term.write_xdr(w)?;
            self.linear_term.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ContractCostParamEntry`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyContractCostParamEntry(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyContractCostParamEntry {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyContractCostParamEntry {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.ext().cmp(&other.ext()))
            .then_with(|| self.const_term().cmp(&other.const_term()))
            .then_with(|| self.linear_term().cmp(&other.linear_term()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyContractCostParamEntry {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyExtensionPoint as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(16).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 16;
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
impl From<LazyHandle> for LazyContractCostParamEntry {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyContractCostParamEntry {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyContractCostParamEntry {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyContractCostParamEntry {
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazyExtensionPoint {
        <LazyExtensionPoint as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `const_term`.
    #[must_use]
    pub fn const_term(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `linear_term`.
    #[must_use]
    pub fn linear_term(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ContractCostParamEntry> for LazyContractCostParamEntry {
    type Error = Error;
    fn try_from(val: &ContractCostParamEntry) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyContractCostParamEntry> for ContractCostParamEntry {
    type Error = Error;
    fn try_from(lazy: &LazyContractCostParamEntry) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
