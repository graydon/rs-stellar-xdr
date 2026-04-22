#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// UInt256Parts is an XDR Struct defined as:
///
/// ```text
/// struct UInt256Parts {
///     uint64 hi_hi;
///     uint64 hi_lo;
///     uint64 lo_hi;
///     uint64 lo_lo;
/// };
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
pub struct UInt256Parts {
    pub hi_hi: u64,
    pub hi_lo: u64,
    pub lo_hi: u64,
    pub lo_lo: u64,
}

impl ReadXdr for UInt256Parts {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                hi_hi: u64::read_xdr(r)?,
                hi_lo: u64::read_xdr(r)?,
                lo_hi: u64::read_xdr(r)?,
                lo_lo: u64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for UInt256Parts {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.hi_hi.write_xdr(w)?;
            self.hi_lo.write_xdr(w)?;
            self.lo_hi.write_xdr(w)?;
            self.lo_lo.write_xdr(w)?;
            Ok(())
        })
    }
}
#[cfg(all(feature = "serde", feature = "alloc"))]
impl<'de> serde::Deserialize<'de> for UInt256Parts {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct UInt256Parts {
            hi_hi: u64,
            hi_lo: u64,
            lo_hi: u64,
            lo_lo: u64,
        }
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum UInt256PartsOrString<'a> {
            Str(&'a str),
            String(String),
            UInt256Parts(UInt256Parts),
        }
        match UInt256PartsOrString::deserialize(deserializer)? {
            UInt256PartsOrString::Str(s) => s.parse().map_err(serde::de::Error::custom),
            UInt256PartsOrString::String(s) => s.parse().map_err(serde::de::Error::custom),
            UInt256PartsOrString::UInt256Parts(UInt256Parts {
                hi_hi,
                hi_lo,
                lo_hi,
                lo_lo,
            }) => Ok(self::UInt256Parts {
                hi_hi,
                hi_lo,
                lo_hi,
                lo_lo,
            }),
        }
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`UInt256Parts`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyUInt256Parts(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyUInt256Parts {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyUInt256Parts {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.hi_hi().cmp(&other.hi_hi()))
            .then_with(|| self.hi_lo().cmp(&other.hi_lo()))
            .then_with(|| self.lo_hi().cmp(&other.lo_hi()))
            .then_with(|| self.lo_lo().cmp(&other.lo_lo()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyUInt256Parts {
    const FIXED_XDR_SIZE: Option<u32> = Some(32);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(32).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        32
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyUInt256Parts {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyUInt256Parts {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyUInt256Parts {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyUInt256Parts {
    /// Access field `hi_hi`.
    #[must_use]
    pub fn hi_hi(&self) -> u64 {
        <u64 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `hi_lo`.
    #[must_use]
    pub fn hi_lo(&self) -> u64 {
        <u64 as LazyXdr>::from_xdr_at(&self.0, 8)
    }
    /// Access field `lo_hi`.
    #[must_use]
    pub fn lo_hi(&self) -> u64 {
        <u64 as LazyXdr>::from_xdr_at(&self.0, 16)
    }
    /// Access field `lo_lo`.
    #[must_use]
    pub fn lo_lo(&self) -> u64 {
        <u64 as LazyXdr>::from_xdr_at(&self.0, 24)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&UInt256Parts> for LazyUInt256Parts {
    type Error = Error;
    fn try_from(val: &UInt256Parts) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyUInt256Parts> for UInt256Parts {
    type Error = Error;
    fn try_from(lazy: &LazyUInt256Parts) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
