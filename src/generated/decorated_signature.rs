#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// DecoratedSignature is an XDR Struct defined as:
///
/// ```text
/// struct DecoratedSignature
/// {
///     SignatureHint hint;  // last 4 bytes of the public key, used as a hint
///     Signature signature; // actual signature
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
pub struct DecoratedSignature {
    pub hint: SignatureHint,
    pub signature: Signature,
}

impl ReadXdr for DecoratedSignature {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                hint: SignatureHint::read_xdr(r)?,
                signature: Signature::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for DecoratedSignature {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.hint.write_xdr(w)?;
            self.signature.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`DecoratedSignature`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyDecoratedSignature(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyDecoratedSignature {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyDecoratedSignature {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.hint().cmp(&other.hint()))
            .then_with(|| self.signature().cmp(&other.signature()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyDecoratedSignature {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(4).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazySignatureHint as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        {
            let field_len = <LazySignature as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += 4;
        pos += <LazySignature as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyDecoratedSignature {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyDecoratedSignature {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyDecoratedSignature {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyDecoratedSignature {
    /// Access field `hint`.
    #[must_use]
    pub fn hint(&self) -> LazySignatureHint {
        <LazySignatureHint as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `signature`.
    #[must_use]
    pub fn signature(&self) -> LazySignature {
        <LazySignature as LazyXdr>::from_xdr_at(&self.0, 4)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&DecoratedSignature> for LazyDecoratedSignature {
    type Error = Error;
    fn try_from(val: &DecoratedSignature) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyDecoratedSignature> for DecoratedSignature {
    type Error = Error;
    fn try_from(lazy: &LazyDecoratedSignature) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
