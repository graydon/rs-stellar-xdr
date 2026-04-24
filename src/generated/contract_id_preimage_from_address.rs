#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ContractIdPreimageFromAddress is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         SCAddress address;
///         uint256 salt;
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
pub struct ContractIdPreimageFromAddress {
    pub address: ScAddress,
    pub salt: Uint256,
}

impl ReadXdr for ContractIdPreimageFromAddress {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                address: ScAddress::read_xdr(r)?,
                salt: Uint256::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ContractIdPreimageFromAddress {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.address.write_xdr(w)?;
            self.salt.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ContractIdPreimageFromAddress`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyContractIdPreimageFromAddress(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyContractIdPreimageFromAddress {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyContractIdPreimageFromAddress {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.address().cmp(&other.address()))
            .then_with(|| self.salt().cmp(&other.salt()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyContractIdPreimageFromAddress {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazyScAddress as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(32).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyUint256 as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyScAddress as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 32;
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
impl From<LazyHandle> for LazyContractIdPreimageFromAddress {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyContractIdPreimageFromAddress {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyContractIdPreimageFromAddress {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyContractIdPreimageFromAddress {
    /// Access field `address`.
    #[must_use]
    pub fn address(&self) -> LazyScAddress {
        <LazyScAddress as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `salt`.
    #[must_use]
    pub fn salt(&self) -> LazyUint256 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyScAddress as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyUint256 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ContractIdPreimageFromAddress> for LazyContractIdPreimageFromAddress {
    type Error = Error;
    fn try_from(val: &ContractIdPreimageFromAddress) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyContractIdPreimageFromAddress> for ContractIdPreimageFromAddress {
    type Error = Error;
    fn try_from(lazy: &LazyContractIdPreimageFromAddress) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
