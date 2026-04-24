#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// HashIdPreimageContractId is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         Hash networkID;
///         ContractIDPreimage contractIDPreimage;
///     }
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
pub struct HashIdPreimageContractId {
    pub network_id: Hash,
    pub contract_id_preimage: ContractIdPreimage,
}

impl ReadXdr for HashIdPreimageContractId {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                network_id: Hash::read_xdr(r)?,
                contract_id_preimage: ContractIdPreimage::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for HashIdPreimageContractId {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.network_id.write_xdr(w)?;
            self.contract_id_preimage.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`HashIdPreimageContractId`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyHashIdPreimageContractId(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyHashIdPreimageContractId {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyHashIdPreimageContractId {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.network_id().cmp(&other.network_id()))
            .then_with(|| {
                self.contract_id_preimage()
                    .cmp(&other.contract_id_preimage())
            })
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyHashIdPreimageContractId {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(32).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyHash as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        {
            let field_len =
                <LazyContractIdPreimage as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += 32;
        pos += <LazyContractIdPreimage as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyHashIdPreimageContractId {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyHashIdPreimageContractId {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyHashIdPreimageContractId {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyHashIdPreimageContractId {
    /// Access field `network_id`.
    #[must_use]
    pub fn network_id(&self) -> LazyHash {
        <LazyHash as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `contract_id_preimage`.
    #[must_use]
    pub fn contract_id_preimage(&self) -> LazyContractIdPreimage {
        <LazyContractIdPreimage as LazyXdr>::from_xdr_at(&self.0, 32)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&HashIdPreimageContractId> for LazyHashIdPreimageContractId {
    type Error = Error;
    fn try_from(val: &HashIdPreimageContractId) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyHashIdPreimageContractId> for HashIdPreimageContractId {
    type Error = Error;
    fn try_from(lazy: &LazyHashIdPreimageContractId) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
