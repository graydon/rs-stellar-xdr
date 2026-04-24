#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// TestNextType is an XDR Struct defined as:
///
/// ```text
/// struct TestNextType
/// {
///     int32 value;
/// };
/// ```
///
#[cfg(feature = "test_feature")]
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
pub struct TestNextType {
    pub value: i32,
}

#[cfg(feature = "test_feature")]
impl ReadXdr for TestNextType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                value: i32::read_xdr(r)?,
            })
        })
    }
}

#[cfg(feature = "test_feature")]
impl WriteXdr for TestNextType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.value.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(all(feature = "alloc", feature = "test_feature"))]
/// Lazy wrapper for [`TestNextType`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyTestNextType(LazyHandle);
#[cfg(all(feature = "alloc", feature = "test_feature"))]
impl PartialOrd for LazyTestNextType {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(all(feature = "alloc", feature = "test_feature"))]
impl Ord for LazyTestNextType {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal.then_with(|| self.value().cmp(&other.value()))
    }
}
#[cfg(all(feature = "alloc", feature = "test_feature"))]
impl LazyXdr for LazyTestNextType {
    const FIXED_XDR_SIZE: Option<u32> = Some(4);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(4).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        4
    }

    fn from_xdr_consume(parent: &LazyHandle, buf: &mut &[u8]) -> Self {
        let len = Self::xdr_len(buf);
        let offset = (parent.len() as usize - buf.len()) as u32;
        let handle = parent.sub_handle(offset, len);
        *buf = &buf[len as usize..];
        Self(handle)
    }
}
#[cfg(all(feature = "alloc", feature = "test_feature"))]
impl From<LazyHandle> for LazyTestNextType {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(all(feature = "alloc", feature = "test_feature"))]
impl AsRef<LazyHandle> for LazyTestNextType {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(all(feature = "alloc", feature = "test_feature"))]
impl TryFrom<Arc<[u8]>> for LazyTestNextType {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(all(feature = "alloc", feature = "test_feature"))]
impl LazyTestNextType {
    /// Access field `value`.
    #[must_use]
    pub fn value(&self) -> i32 {
        <i32 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
}
#[cfg(all(feature = "alloc", feature = "std", feature = "test_feature"))]
impl TryFrom<&TestNextType> for LazyTestNextType {
    type Error = Error;
    fn try_from(val: &TestNextType) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std", feature = "test_feature"))]
impl TryFrom<&LazyTestNextType> for TestNextType {
    type Error = Error;
    fn try_from(lazy: &LazyTestNextType) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
