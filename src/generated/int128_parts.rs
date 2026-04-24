#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// Int128Parts is an XDR Struct defined as:
///
/// ```text
/// struct Int128Parts {
///     int64 hi;
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
pub struct Int128Parts {
    pub hi: i64,
    pub lo: u64,
}

impl ReadXdr for Int128Parts {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                hi: i64::read_xdr(r)?,
                lo: u64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for Int128Parts {
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
impl<'de> serde::Deserialize<'de> for Int128Parts {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct Int128Parts {
            hi: i64,
            lo: u64,
        }
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Int128PartsOrString<'a> {
            Str(&'a str),
            String(String),
            Int128Parts(Int128Parts),
        }
        match Int128PartsOrString::deserialize(deserializer)? {
            Int128PartsOrString::Str(s) => s.parse().map_err(serde::de::Error::custom),
            Int128PartsOrString::String(s) => s.parse().map_err(serde::de::Error::custom),
            Int128PartsOrString::Int128Parts(Int128Parts { hi, lo }) => {
                Ok(self::Int128Parts { hi, lo })
            }
        }
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`Int128Parts`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyInt128Parts(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyInt128Parts {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyInt128Parts {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.hi().cmp(&other.hi()))
            .then_with(|| self.lo().cmp(&other.lo()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyInt128Parts {
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

    fn from_xdr_consume(parent: &LazyHandle, buf: &mut &[u8]) -> Self {
        let len = Self::xdr_len(buf);
        let offset = (parent.len() as usize - buf.len()) as u32;
        let handle = parent.sub_handle(offset, len);
        *buf = &buf[len as usize..];
        Self(handle)
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyInt128Parts {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyInt128Parts {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyInt128Parts {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyInt128Parts {
    /// Access field `hi`.
    #[must_use]
    pub fn hi(&self) -> i64 {
        <i64 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `lo`.
    #[must_use]
    pub fn lo(&self) -> u64 {
        <u64 as LazyXdr>::from_xdr_at(&self.0, 8)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&Int128Parts> for LazyInt128Parts {
    type Error = Error;
    fn try_from(val: &Int128Parts) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyInt128Parts> for Int128Parts {
    type Error = Error;
    fn try_from(lazy: &LazyInt128Parts) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
