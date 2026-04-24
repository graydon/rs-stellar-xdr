#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// EncryptedBody is an XDR Typedef defined as:
///
/// ```text
/// typedef opaque EncryptedBody<64000>;
/// ```
///
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug)]
pub struct EncryptedBody(pub BytesM<64000>);

impl From<EncryptedBody> for BytesM<64000> {
    #[must_use]
    fn from(x: EncryptedBody) -> Self {
        x.0
    }
}

impl From<BytesM<64000>> for EncryptedBody {
    #[must_use]
    fn from(x: BytesM<64000>) -> Self {
        EncryptedBody(x)
    }
}

impl AsRef<BytesM<64000>> for EncryptedBody {
    #[must_use]
    fn as_ref(&self) -> &BytesM<64000> {
        &self.0
    }
}

impl ReadXdr for EncryptedBody {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = BytesM::<64000>::read_xdr(r)?;
            let v = EncryptedBody(i);
            Ok(v)
        })
    }
}

impl WriteXdr for EncryptedBody {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for EncryptedBody {
    type Target = BytesM<64000>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<EncryptedBody> for Vec<u8> {
    #[must_use]
    fn from(x: EncryptedBody) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<u8>> for EncryptedBody {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self, Error> {
        Ok(EncryptedBody(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for EncryptedBody {
    type Error = Error;
    fn try_from(x: &Vec<u8>) -> Result<Self, Error> {
        Ok(EncryptedBody(x.try_into()?))
    }
}

impl AsRef<Vec<u8>> for EncryptedBody {
    #[must_use]
    fn as_ref(&self) -> &Vec<u8> {
        &self.0 .0
    }
}

impl AsRef<[u8]> for EncryptedBody {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        self.0 .0
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`EncryptedBody`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyEncryptedBody(LazyBytesM<64000>);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyEncryptedBody {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyEncryptedBody {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyEncryptedBody {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        <LazyBytesM<64000> as LazyXdr>::xdr_validate(buf, depth)
    }

    #[inline]
    fn xdr_len(buf: &[u8]) -> u32 {
        <LazyBytesM<64000> as LazyXdr>::xdr_len(buf)
    }

    fn from_xdr_consume(parent: &LazyHandle, buf: &mut &[u8]) -> Self {
        Self(<LazyBytesM<64000> as LazyXdr>::from_xdr_consume(
            parent, buf,
        ))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyEncryptedBody {
    fn from(h: LazyHandle) -> Self {
        Self(<LazyBytesM<64000>>::from(h))
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyEncryptedBody {
    fn as_ref(&self) -> &LazyHandle {
        self.0.as_ref()
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyEncryptedBody {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(<LazyBytesM<64000>>::from(LazyHandle::from_arc(
            buf, 0, len,
        ))))
    }
}
#[cfg(feature = "alloc")]
impl core::ops::Deref for LazyEncryptedBody {
    type Target = LazyBytesM<64000>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&EncryptedBody> for LazyEncryptedBody {
    type Error = Error;
    fn try_from(val: &EncryptedBody) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyEncryptedBody> for EncryptedBody {
    type Error = Error;
    fn try_from(lazy: &LazyEncryptedBody) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
