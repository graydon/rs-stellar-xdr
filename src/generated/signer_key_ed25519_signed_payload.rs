#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SignerKeyEd25519SignedPayload is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         /* Public key that must sign the payload. */
///         uint256 ed25519;
///         /* Payload to be raw signed by ed25519. */
///         opaque payload<64>;
///     }
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay)
)]
pub struct SignerKeyEd25519SignedPayload {
    pub ed25519: Uint256,
    pub payload: BytesM<64>,
}

impl ReadXdr for SignerKeyEd25519SignedPayload {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ed25519: Uint256::read_xdr(r)?,
                payload: BytesM::<64>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SignerKeyEd25519SignedPayload {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ed25519.write_xdr(w)?;
            self.payload.write_xdr(w)?;
            Ok(())
        })
    }
}
#[cfg(all(feature = "serde", feature = "alloc"))]
impl<'de> serde::Deserialize<'de> for SignerKeyEd25519SignedPayload {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct SignerKeyEd25519SignedPayload {
            ed25519: Uint256,
            payload: BytesM<64>,
        }
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum SignerKeyEd25519SignedPayloadOrString<'a> {
            Str(&'a str),
            String(String),
            SignerKeyEd25519SignedPayload(SignerKeyEd25519SignedPayload),
        }
        match SignerKeyEd25519SignedPayloadOrString::deserialize(deserializer)? {
            SignerKeyEd25519SignedPayloadOrString::Str(s) => {
                s.parse().map_err(serde::de::Error::custom)
            }
            SignerKeyEd25519SignedPayloadOrString::String(s) => {
                s.parse().map_err(serde::de::Error::custom)
            }
            SignerKeyEd25519SignedPayloadOrString::SignerKeyEd25519SignedPayload(
                SignerKeyEd25519SignedPayload { ed25519, payload },
            ) => Ok(self::SignerKeyEd25519SignedPayload { ed25519, payload }),
        }
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`SignerKeyEd25519SignedPayload`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySignerKeyEd25519SignedPayload(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazySignerKeyEd25519SignedPayload {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazySignerKeyEd25519SignedPayload {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.ed25519().cmp(&other.ed25519()))
            .then_with(|| self.payload().cmp(&other.payload()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazySignerKeyEd25519SignedPayload {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(32).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyUint256 as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        {
            let field_len = <LazyBytesM<64> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += 32;
        pos += <LazyBytesM<64> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazySignerKeyEd25519SignedPayload {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazySignerKeyEd25519SignedPayload {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazySignerKeyEd25519SignedPayload {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazySignerKeyEd25519SignedPayload {
    /// Access field `ed25519`.
    #[must_use]
    pub fn ed25519(&self) -> LazyUint256 {
        <LazyUint256 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `payload`.
    #[must_use]
    pub fn payload(&self) -> LazyBytesM<64> {
        <LazyBytesM<64> as LazyXdr>::from_xdr_at(&self.0, 32)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&SignerKeyEd25519SignedPayload> for LazySignerKeyEd25519SignedPayload {
    type Error = Error;
    fn try_from(val: &SignerKeyEd25519SignedPayload) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazySignerKeyEd25519SignedPayload> for SignerKeyEd25519SignedPayload {
    type Error = Error;
    fn try_from(lazy: &LazySignerKeyEd25519SignedPayload) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
