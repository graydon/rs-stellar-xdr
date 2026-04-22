#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SorobanDelegateSignature is an XDR Struct defined as:
///
/// ```text
/// struct SorobanDelegateSignature {
///     SCAddress address;
///     SCVal signature;
///     SorobanDelegateSignature nestedDelegates<>;
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
pub struct SorobanDelegateSignature {
    pub address: ScAddress,
    pub signature: ScVal,
    pub nested_delegates: VecM<SorobanDelegateSignature>,
}

#[cfg(feature = "cap_0071")]
impl ReadXdr for SorobanDelegateSignature {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                address: ScAddress::read_xdr(r)?,
                signature: ScVal::read_xdr(r)?,
                nested_delegates: VecM::<SorobanDelegateSignature>::read_xdr(r)?,
            })
        })
    }
}

#[cfg(feature = "cap_0071")]
impl WriteXdr for SorobanDelegateSignature {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.address.write_xdr(w)?;
            self.signature.write_xdr(w)?;
            self.nested_delegates.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(all(feature = "alloc", feature = "cap_0071"))]
/// Lazy wrapper for [`SorobanDelegateSignature`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySorobanDelegateSignature(LazyHandle);
#[cfg(all(feature = "alloc", feature = "cap_0071"))]
impl PartialOrd for LazySorobanDelegateSignature {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(all(feature = "alloc", feature = "cap_0071"))]
impl Ord for LazySorobanDelegateSignature {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.address().cmp(&other.address()))
            .then_with(|| self.signature().cmp(&other.signature()))
            .then_with(|| self.nested_delegates().cmp(&other.nested_delegates()))
    }
}
#[cfg(all(feature = "alloc", feature = "cap_0071"))]
impl LazyXdr for LazySorobanDelegateSignature {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazyScAddress as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyScVal as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
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
        pos += <LazyScAddress as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyScVal as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazySorobanDelegateSignature {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(all(feature = "alloc", feature = "cap_0071"))]
impl AsRef<LazyHandle> for LazySorobanDelegateSignature {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(all(feature = "alloc", feature = "cap_0071"))]
impl TryFrom<Arc<[u8]>> for LazySorobanDelegateSignature {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(all(feature = "alloc", feature = "cap_0071"))]
impl LazySorobanDelegateSignature {
    /// Access field `address`.
    #[must_use]
    pub fn address(&self) -> LazyScAddress {
        <LazyScAddress as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `signature`.
    #[must_use]
    pub fn signature(&self) -> LazyScVal {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyScAddress as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyScVal as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `nested_delegates`.
    #[must_use]
    pub fn nested_delegates(&self) -> LazyVecM<LazySorobanDelegateSignature> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyScAddress as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyScVal as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazySorobanDelegateSignature> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std", feature = "cap_0071"))]
impl TryFrom<&SorobanDelegateSignature> for LazySorobanDelegateSignature {
    type Error = Error;
    fn try_from(val: &SorobanDelegateSignature) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std", feature = "cap_0071"))]
impl TryFrom<&LazySorobanDelegateSignature> for SorobanDelegateSignature {
    type Error = Error;
    fn try_from(lazy: &LazySorobanDelegateSignature) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
