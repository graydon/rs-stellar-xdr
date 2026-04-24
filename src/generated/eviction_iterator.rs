#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// EvictionIterator is an XDR Struct defined as:
///
/// ```text
/// struct EvictionIterator {
///     uint32 bucketListLevel;
///     bool isCurrBucket;
///     uint64 bucketFileOffset;
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
pub struct EvictionIterator {
    pub bucket_list_level: u32,
    pub is_curr_bucket: bool,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub bucket_file_offset: u64,
}

impl ReadXdr for EvictionIterator {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                bucket_list_level: u32::read_xdr(r)?,
                is_curr_bucket: bool::read_xdr(r)?,
                bucket_file_offset: u64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for EvictionIterator {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.bucket_list_level.write_xdr(w)?;
            self.is_curr_bucket.write_xdr(w)?;
            self.bucket_file_offset.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`EvictionIterator`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyEvictionIterator(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyEvictionIterator {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyEvictionIterator {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.bucket_list_level().cmp(&other.bucket_list_level()))
            .then_with(|| self.is_curr_bucket().cmp(&other.is_curr_bucket()))
            .then_with(|| self.bucket_file_offset().cmp(&other.bucket_file_offset()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyEvictionIterator {
    const FIXED_XDR_SIZE: Option<u32> = Some(16);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(16).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <bool as LazyXdr>::xdr_validate(&buf[(pos + 4) as usize..], depth)?;
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
impl From<LazyHandle> for LazyEvictionIterator {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyEvictionIterator {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyEvictionIterator {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyEvictionIterator {
    /// Access field `bucket_list_level`.
    #[must_use]
    pub fn bucket_list_level(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `is_curr_bucket`.
    #[must_use]
    pub fn is_curr_bucket(&self) -> bool {
        <bool as LazyXdr>::from_xdr_at(&self.0, 4)
    }
    /// Access field `bucket_file_offset`.
    #[must_use]
    pub fn bucket_file_offset(&self) -> u64 {
        <u64 as LazyXdr>::from_xdr_at(&self.0, 8)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&EvictionIterator> for LazyEvictionIterator {
    type Error = Error;
    fn try_from(val: &EvictionIterator) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyEvictionIterator> for EvictionIterator {
    type Error = Error;
    fn try_from(lazy: &LazyEvictionIterator) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
