#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// AuthenticatedMessageV0 is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         uint64 sequence;
///         StellarMessage message;
///         HmacSha256Mac mac;
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
pub struct AuthenticatedMessageV0 {
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub sequence: u64,
    pub message: StellarMessage,
    pub mac: HmacSha256Mac,
}

impl ReadXdr for AuthenticatedMessageV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                sequence: u64::read_xdr(r)?,
                message: StellarMessage::read_xdr(r)?,
                mac: HmacSha256Mac::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for AuthenticatedMessageV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.sequence.write_xdr(w)?;
            self.message.write_xdr(w)?;
            self.mac.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`AuthenticatedMessageV0`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyAuthenticatedMessageV0(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyAuthenticatedMessageV0 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyAuthenticatedMessageV0 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.sequence().cmp(&other.sequence()))
            .then_with(|| self.message().cmp(&other.message()))
            .then_with(|| self.mac().cmp(&other.mac()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyAuthenticatedMessageV0 {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(8).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        {
            let field_len =
                <LazyStellarMessage as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(32).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyHmacSha256Mac as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += 8;
        pos += <LazyStellarMessage as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyAuthenticatedMessageV0 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyAuthenticatedMessageV0 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyAuthenticatedMessageV0 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyAuthenticatedMessageV0 {
    /// Access field `sequence`.
    #[must_use]
    pub fn sequence(&self) -> u64 {
        <u64 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `message`.
    #[must_use]
    pub fn message(&self) -> LazyStellarMessage {
        <LazyStellarMessage as LazyXdr>::from_xdr_at(&self.0, 8)
    }
    /// Access field `mac`.
    #[must_use]
    pub fn mac(&self) -> LazyHmacSha256Mac {
        let buf = self.0.as_slice();
        let mut pos: u32 = 8;
        pos += <LazyStellarMessage as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyHmacSha256Mac as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&AuthenticatedMessageV0> for LazyAuthenticatedMessageV0 {
    type Error = Error;
    fn try_from(val: &AuthenticatedMessageV0) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyAuthenticatedMessageV0> for AuthenticatedMessageV0 {
    type Error = Error;
    fn try_from(lazy: &LazyAuthenticatedMessageV0) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
