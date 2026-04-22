#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// UInt128Parts is an XDR Struct defined as:
///
/// ```text
/// struct UInt128Parts {
///     uint64 hi;
///     uint64 lo;
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
pub struct UInt128Parts {
    pub hi: u64,
    pub lo: u64,
}

impl ReadXdr for UInt128Parts {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                hi: u64::read_xdr(r)?,
                lo: u64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for UInt128Parts {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.hi.write_xdr(w)?;
            self.lo.write_xdr(w)?;
            Ok(())
        })
    }
}
#[cfg(all(feature = "serde", feature = "alloc"))]
impl<'de> serde::Deserialize<'de> for UInt128Parts {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct UInt128Parts {
            hi: u64,
            lo: u64,
        }
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum UInt128PartsOrString<'a> {
            Str(&'a str),
            String(String),
            UInt128Parts(UInt128Parts),
        }
        match UInt128PartsOrString::deserialize(deserializer)? {
            UInt128PartsOrString::Str(s) => s.parse().map_err(serde::de::Error::custom),
            UInt128PartsOrString::String(s) => s.parse().map_err(serde::de::Error::custom),
            UInt128PartsOrString::UInt128Parts(UInt128Parts { hi, lo }) => {
                Ok(self::UInt128Parts { hi, lo })
            }
        }
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`UInt128Parts`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyUInt128Parts(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyUInt128Parts {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyUInt128Parts {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.hi().cmp(&other.hi()))
            .then_with(|| self.lo().cmp(&other.lo()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyUInt128Parts {
    const FIXED_XDR_SIZE: Option<u32> = Some(16);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(16).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        16
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyUInt128Parts {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyUInt128Parts {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyUInt128Parts {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyUInt128Parts {
    /// Access field `hi`.
    #[must_use]
    pub fn hi(&self) -> u64 {
        <u64 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `lo`.
    #[must_use]
    pub fn lo(&self) -> u64 {
        <u64 as LazyXdr>::from_xdr_at(&self.0, 8)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&UInt128Parts> for LazyUInt128Parts {
    type Error = Error;
    fn try_from(val: &UInt128Parts) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyUInt128Parts> for UInt128Parts {
    type Error = Error;
    fn try_from(lazy: &LazyUInt128Parts) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
