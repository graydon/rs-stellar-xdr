#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// AccountId is an XDR Typedef defined as:
///
/// ```text
/// typedef PublicKey AccountID;
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
pub struct AccountId(pub PublicKey);

impl From<AccountId> for PublicKey {
    #[must_use]
    fn from(x: AccountId) -> Self {
        x.0
    }
}

impl From<PublicKey> for AccountId {
    #[must_use]
    fn from(x: PublicKey) -> Self {
        AccountId(x)
    }
}

impl AsRef<PublicKey> for AccountId {
    #[must_use]
    fn as_ref(&self) -> &PublicKey {
        &self.0
    }
}

impl ReadXdr for AccountId {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = PublicKey::read_xdr(r)?;
            let v = AccountId(i);
            Ok(v)
        })
    }
}

impl WriteXdr for AccountId {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`AccountId`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyAccountId(LazyPublicKey);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyAccountId {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyAccountId {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyAccountId {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        <LazyPublicKey as LazyXdr>::xdr_validate(buf, depth)
    }

    #[inline]
    fn xdr_len(buf: &[u8]) -> u32 {
        <LazyPublicKey as LazyXdr>::xdr_len(buf)
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        Self(<LazyPublicKey as LazyXdr>::from_xdr_at(parent, offset))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyAccountId {
    fn from(h: LazyHandle) -> Self {
        Self(<LazyPublicKey>::from(h))
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyAccountId {
    fn as_ref(&self) -> &LazyHandle {
        self.0.as_ref()
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyAccountId {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(<LazyPublicKey>::from(LazyHandle::from_arc(
            buf, 0, len,
        ))))
    }
}
#[cfg(feature = "alloc")]
impl core::ops::Deref for LazyAccountId {
    type Target = LazyPublicKey;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&AccountId> for LazyAccountId {
    type Error = Error;
    fn try_from(val: &AccountId) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyAccountId> for AccountId {
    type Error = Error;
    fn try_from(lazy: &LazyAccountId) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
