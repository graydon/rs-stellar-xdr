#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// AuthCert is an XDR Struct defined as:
///
/// ```text
/// struct AuthCert
/// {
///     Curve25519Public pubkey;
///     uint64 expiration;
///     Signature sig;
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
pub struct AuthCert {
    pub pubkey: Curve25519Public,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub expiration: u64,
    pub sig: Signature,
}

impl ReadXdr for AuthCert {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                pubkey: Curve25519Public::read_xdr(r)?,
                expiration: u64::read_xdr(r)?,
                sig: Signature::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for AuthCert {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.pubkey.write_xdr(w)?;
            self.expiration.write_xdr(w)?;
            self.sig.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`AuthCert`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyAuthCert(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyAuthCert {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyAuthCert {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.pubkey().cmp(&other.pubkey()))
            .then_with(|| self.expiration().cmp(&other.expiration()))
            .then_with(|| self.sig().cmp(&other.sig()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyAuthCert {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(40).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyCurve25519Public as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        {
            let field_len = <LazySignature as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += 40;
        pos += <LazySignature as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyAuthCert {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyAuthCert {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyAuthCert {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyAuthCert {
    /// Access field `pubkey`.
    #[must_use]
    pub fn pubkey(&self) -> LazyCurve25519Public {
        <LazyCurve25519Public as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `expiration`.
    #[must_use]
    pub fn expiration(&self) -> u64 {
        <u64 as LazyXdr>::from_xdr_at(&self.0, 32)
    }
    /// Access field `sig`.
    #[must_use]
    pub fn sig(&self) -> LazySignature {
        <LazySignature as LazyXdr>::from_xdr_at(&self.0, 40)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&AuthCert> for LazyAuthCert {
    type Error = Error;
    fn try_from(val: &AuthCert) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyAuthCert> for AuthCert {
    type Error = Error;
    fn try_from(lazy: &LazyAuthCert) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
