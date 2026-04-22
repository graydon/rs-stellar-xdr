#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SequenceNumber is an XDR Typedef defined as:
///
/// ```text
/// typedef int64 SequenceNumber;
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
pub struct SequenceNumber(
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub i64,
);

impl From<SequenceNumber> for i64 {
    #[must_use]
    fn from(x: SequenceNumber) -> Self {
        x.0
    }
}

impl From<i64> for SequenceNumber {
    #[must_use]
    fn from(x: i64) -> Self {
        SequenceNumber(x)
    }
}

impl AsRef<i64> for SequenceNumber {
    #[must_use]
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

impl ReadXdr for SequenceNumber {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = i64::read_xdr(r)?;
            let v = SequenceNumber(i);
            Ok(v)
        })
    }
}

impl WriteXdr for SequenceNumber {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`SequenceNumber`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySequenceNumber(i64);
#[cfg(feature = "alloc")]
impl PartialOrd for LazySequenceNumber {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazySequenceNumber {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazySequenceNumber {
    const FIXED_XDR_SIZE: Option<u32> = Some(8);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        <i64 as LazyXdr>::xdr_validate(buf, depth)
    }

    #[inline]
    fn xdr_len(buf: &[u8]) -> u32 {
        <i64 as LazyXdr>::xdr_len(buf)
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        Self(<i64 as LazyXdr>::from_xdr_at(parent, offset))
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazySequenceNumber {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self::from_xdr_at(&LazyHandle::from_arc(buf, 0, len), 0))
    }
}
#[cfg(feature = "alloc")]
impl core::ops::Deref for LazySequenceNumber {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&SequenceNumber> for LazySequenceNumber {
    type Error = Error;
    fn try_from(val: &SequenceNumber) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl From<&LazySequenceNumber> for SequenceNumber {
    fn from(lazy: &LazySequenceNumber) -> Self {
        SequenceNumber(**lazy)
    }
}
