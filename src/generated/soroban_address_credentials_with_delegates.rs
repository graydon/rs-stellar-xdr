#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SorobanAddressCredentialsWithDelegates is an XDR Struct defined as:
///
/// ```text
/// struct SorobanAddressCredentialsWithDelegates
/// {
///     SorobanAddressCredentials addressCredentials;
///     SorobanDelegateSignature delegates<>;
/// };
/// ```
///
#[cfg(feature = "cap_0071")]
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
pub struct SorobanAddressCredentialsWithDelegates {
    pub address_credentials: SorobanAddressCredentials,
    pub delegates: VecM<SorobanDelegateSignature>,
}

#[cfg(feature = "cap_0071")]
impl ReadXdr for SorobanAddressCredentialsWithDelegates {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                address_credentials: SorobanAddressCredentials::read_xdr(r)?,
                delegates: VecM::<SorobanDelegateSignature>::read_xdr(r)?,
            })
        })
    }
}

#[cfg(feature = "cap_0071")]
impl WriteXdr for SorobanAddressCredentialsWithDelegates {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.address_credentials.write_xdr(w)?;
            self.delegates.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(all(feature = "alloc", feature = "cap_0071"))]
/// Lazy wrapper for [`SorobanAddressCredentialsWithDelegates`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySorobanAddressCredentialsWithDelegates(LazyHandle);
#[cfg(all(feature = "alloc", feature = "cap_0071"))]
impl PartialOrd for LazySorobanAddressCredentialsWithDelegates {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(all(feature = "alloc", feature = "cap_0071"))]
impl Ord for LazySorobanAddressCredentialsWithDelegates {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.address_credentials().cmp(&other.address_credentials()))
            .then_with(|| self.delegates().cmp(&other.delegates()))
    }
}
#[cfg(all(feature = "alloc", feature = "cap_0071"))]
impl LazyXdr for LazySorobanAddressCredentialsWithDelegates {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazySorobanAddressCredentials as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyVecM<LazySorobanDelegateSignature> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazySorobanAddressCredentials as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazySorobanDelegateSignature> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(all(feature = "alloc", feature = "cap_0071"))]
impl From<LazyHandle> for LazySorobanAddressCredentialsWithDelegates {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(all(feature = "alloc", feature = "cap_0071"))]
impl AsRef<LazyHandle> for LazySorobanAddressCredentialsWithDelegates {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(all(feature = "alloc", feature = "cap_0071"))]
impl TryFrom<Arc<[u8]>> for LazySorobanAddressCredentialsWithDelegates {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(all(feature = "alloc", feature = "cap_0071"))]
impl LazySorobanAddressCredentialsWithDelegates {
    /// Access field `address_credentials`.
    #[must_use]
    pub fn address_credentials(&self) -> LazySorobanAddressCredentials {
        <LazySorobanAddressCredentials as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `delegates`.
    #[must_use]
    pub fn delegates(&self) -> LazyVecM<LazySorobanDelegateSignature> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazySorobanAddressCredentials as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazySorobanDelegateSignature> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std", feature = "cap_0071"))]
impl TryFrom<&SorobanAddressCredentialsWithDelegates>
    for LazySorobanAddressCredentialsWithDelegates
{
    type Error = Error;
    fn try_from(val: &SorobanAddressCredentialsWithDelegates) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std", feature = "cap_0071"))]
impl TryFrom<&LazySorobanAddressCredentialsWithDelegates>
    for SorobanAddressCredentialsWithDelegates
{
    type Error = Error;
    fn try_from(lazy: &LazySorobanAddressCredentialsWithDelegates) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
