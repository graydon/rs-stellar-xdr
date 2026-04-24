#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SorobanAuthorizationEntry is an XDR Struct defined as:
///
/// ```text
/// struct SorobanAuthorizationEntry
/// {
///     SorobanCredentials credentials;
///     SorobanAuthorizedInvocation rootInvocation;
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
pub struct SorobanAuthorizationEntry {
    pub credentials: SorobanCredentials,
    pub root_invocation: SorobanAuthorizedInvocation,
}

impl ReadXdr for SorobanAuthorizationEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                credentials: SorobanCredentials::read_xdr(r)?,
                root_invocation: SorobanAuthorizedInvocation::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SorobanAuthorizationEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.credentials.write_xdr(w)?;
            self.root_invocation.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`SorobanAuthorizationEntry`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySorobanAuthorizationEntry(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazySorobanAuthorizationEntry {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazySorobanAuthorizationEntry {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.credentials().cmp(&other.credentials()))
            .then_with(|| self.root_invocation().cmp(&other.root_invocation()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazySorobanAuthorizationEntry {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazySorobanCredentials as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazySorobanAuthorizedInvocation as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazySorobanCredentials as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazySorobanAuthorizedInvocation as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazySorobanAuthorizationEntry {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazySorobanAuthorizationEntry {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazySorobanAuthorizationEntry {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazySorobanAuthorizationEntry {
    /// Access field `credentials`.
    #[must_use]
    pub fn credentials(&self) -> LazySorobanCredentials {
        <LazySorobanCredentials as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `root_invocation`.
    #[must_use]
    pub fn root_invocation(&self) -> LazySorobanAuthorizedInvocation {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazySorobanCredentials as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazySorobanAuthorizedInvocation as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&SorobanAuthorizationEntry> for LazySorobanAuthorizationEntry {
    type Error = Error;
    fn try_from(val: &SorobanAuthorizationEntry) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazySorobanAuthorizationEntry> for SorobanAuthorizationEntry {
    type Error = Error;
    fn try_from(lazy: &LazySorobanAuthorizationEntry) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
