#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// TimePoint is an XDR Typedef defined as:
///
/// ```text
/// typedef uint64 TimePoint;
/// ```
///
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug)]
pub struct TimePoint(
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub u64,
);

impl From<TimePoint> for u64 {
    #[must_use]
    fn from(x: TimePoint) -> Self {
        x.0
    }
}

impl From<u64> for TimePoint {
    #[must_use]
    fn from(x: u64) -> Self {
        TimePoint(x)
    }
}

impl AsRef<u64> for TimePoint {
    #[must_use]
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

impl ReadXdr for TimePoint {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = u64::read_xdr(r)?;
            let v = TimePoint(i);
            Ok(v)
        })
    }
}

impl WriteXdr for TimePoint {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`TimePoint`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyTimePoint(u64);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyTimePoint {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyTimePoint {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyTimePoint {
    const FIXED_XDR_SIZE: Option<u32> = Some(8);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        <u64 as LazyXdr>::xdr_validate(buf, depth)
    }

    #[inline]
    fn xdr_len(buf: &[u8]) -> u32 {
        <u64 as LazyXdr>::xdr_len(buf)
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        Self(<u64 as LazyXdr>::from_xdr_at(parent, offset))
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyTimePoint {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self::from_xdr_at(&LazyHandle::from_arc(buf, 0, len), 0))
    }
}
#[cfg(feature = "alloc")]
impl core::ops::Deref for LazyTimePoint {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&TimePoint> for LazyTimePoint {
    type Error = Error;
    fn try_from(val: &TimePoint) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl From<&LazyTimePoint> for TimePoint {
    fn from(lazy: &LazyTimePoint) -> Self {
        TimePoint(**lazy)
    }
}
