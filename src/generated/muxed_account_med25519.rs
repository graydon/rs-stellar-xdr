#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// MuxedAccountMed25519 is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         uint64 id;
///         uint256 ed25519;
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
pub struct MuxedAccountMed25519 {
    pub id: u64,
    pub ed25519: Uint256,
}

impl ReadXdr for MuxedAccountMed25519 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                id: u64::read_xdr(r)?,
                ed25519: Uint256::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for MuxedAccountMed25519 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.id.write_xdr(w)?;
            self.ed25519.write_xdr(w)?;
            Ok(())
        })
    }
}
#[cfg(all(feature = "serde", feature = "alloc"))]
impl<'de> serde::Deserialize<'de> for MuxedAccountMed25519 {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct MuxedAccountMed25519 {
            id: u64,
            ed25519: Uint256,
        }
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum MuxedAccountMed25519OrString<'a> {
            Str(&'a str),
            String(String),
            MuxedAccountMed25519(MuxedAccountMed25519),
        }
        match MuxedAccountMed25519OrString::deserialize(deserializer)? {
            MuxedAccountMed25519OrString::Str(s) => s.parse().map_err(serde::de::Error::custom),
            MuxedAccountMed25519OrString::String(s) => s.parse().map_err(serde::de::Error::custom),
            MuxedAccountMed25519OrString::MuxedAccountMed25519(MuxedAccountMed25519 {
                id,
                ed25519,
            }) => Ok(self::MuxedAccountMed25519 { id, ed25519 }),
        }
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`MuxedAccountMed25519`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyMuxedAccountMed25519(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyMuxedAccountMed25519 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyMuxedAccountMed25519 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.id().cmp(&other.id()))
            .then_with(|| self.ed25519().cmp(&other.ed25519()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyMuxedAccountMed25519 {
    const FIXED_XDR_SIZE: Option<u32> = Some(40);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(40).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyUint256 as LazyXdr>::xdr_validate(&buf[(pos + 8) as usize..], depth)?;
        pos = next_pos;
        Ok(pos)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        40
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyMuxedAccountMed25519 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyMuxedAccountMed25519 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyMuxedAccountMed25519 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyMuxedAccountMed25519 {
    /// Access field `id`.
    #[must_use]
    pub fn id(&self) -> u64 {
        <u64 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `ed25519`.
    #[must_use]
    pub fn ed25519(&self) -> LazyUint256 {
        <LazyUint256 as LazyXdr>::from_xdr_at(&self.0, 8)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&MuxedAccountMed25519> for LazyMuxedAccountMed25519 {
    type Error = Error;
    fn try_from(val: &MuxedAccountMed25519) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyMuxedAccountMed25519> for MuxedAccountMed25519 {
    type Error = Error;
    fn try_from(lazy: &LazyMuxedAccountMed25519) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
