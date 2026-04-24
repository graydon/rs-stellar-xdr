#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// TimeSlicedPeerDataList is an XDR Typedef defined as:
///
/// ```text
/// typedef TimeSlicedPeerData TimeSlicedPeerDataList<25>;
/// ```
///
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug)]
pub struct TimeSlicedPeerDataList(pub VecM<TimeSlicedPeerData, 25>);

impl From<TimeSlicedPeerDataList> for VecM<TimeSlicedPeerData, 25> {
    #[must_use]
    fn from(x: TimeSlicedPeerDataList) -> Self {
        x.0
    }
}

impl From<VecM<TimeSlicedPeerData, 25>> for TimeSlicedPeerDataList {
    #[must_use]
    fn from(x: VecM<TimeSlicedPeerData, 25>) -> Self {
        TimeSlicedPeerDataList(x)
    }
}

impl AsRef<VecM<TimeSlicedPeerData, 25>> for TimeSlicedPeerDataList {
    #[must_use]
    fn as_ref(&self) -> &VecM<TimeSlicedPeerData, 25> {
        &self.0
    }
}

impl ReadXdr for TimeSlicedPeerDataList {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = VecM::<TimeSlicedPeerData, 25>::read_xdr(r)?;
            let v = TimeSlicedPeerDataList(i);
            Ok(v)
        })
    }
}

impl WriteXdr for TimeSlicedPeerDataList {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for TimeSlicedPeerDataList {
    type Target = VecM<TimeSlicedPeerData, 25>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TimeSlicedPeerDataList> for Vec<TimeSlicedPeerData> {
    #[must_use]
    fn from(x: TimeSlicedPeerDataList) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<TimeSlicedPeerData>> for TimeSlicedPeerDataList {
    type Error = Error;
    fn try_from(x: Vec<TimeSlicedPeerData>) -> Result<Self, Error> {
        Ok(TimeSlicedPeerDataList(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<TimeSlicedPeerData>> for TimeSlicedPeerDataList {
    type Error = Error;
    fn try_from(x: &Vec<TimeSlicedPeerData>) -> Result<Self, Error> {
        Ok(TimeSlicedPeerDataList(x.try_into()?))
    }
}

impl AsRef<Vec<TimeSlicedPeerData>> for TimeSlicedPeerDataList {
    #[must_use]
    fn as_ref(&self) -> &Vec<TimeSlicedPeerData> {
        &self.0 .0
    }
}

impl AsRef<[TimeSlicedPeerData]> for TimeSlicedPeerDataList {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[TimeSlicedPeerData] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[TimeSlicedPeerData] {
        self.0 .0
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`TimeSlicedPeerDataList`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyTimeSlicedPeerDataList(LazyVecM<LazyTimeSlicedPeerData, 25>);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyTimeSlicedPeerDataList {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyTimeSlicedPeerDataList {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyTimeSlicedPeerDataList {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        <LazyVecM<LazyTimeSlicedPeerData, 25> as LazyXdr>::xdr_validate(buf, depth)
    }

    #[inline]
    fn xdr_len(buf: &[u8]) -> u32 {
        <LazyVecM<LazyTimeSlicedPeerData, 25> as LazyXdr>::xdr_len(buf)
    }

    fn from_xdr_consume(parent: &LazyHandle, buf: &mut &[u8]) -> Self {
        Self(<LazyVecM<LazyTimeSlicedPeerData, 25> as LazyXdr>::from_xdr_consume(parent, buf))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyTimeSlicedPeerDataList {
    fn from(h: LazyHandle) -> Self {
        Self(<LazyVecM<LazyTimeSlicedPeerData, 25>>::from(h))
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyTimeSlicedPeerDataList {
    fn as_ref(&self) -> &LazyHandle {
        self.0.as_ref()
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyTimeSlicedPeerDataList {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(<LazyVecM<LazyTimeSlicedPeerData, 25>>::from(
            LazyHandle::from_arc(buf, 0, len),
        )))
    }
}
#[cfg(feature = "alloc")]
impl core::ops::Deref for LazyTimeSlicedPeerDataList {
    type Target = LazyVecM<LazyTimeSlicedPeerData, 25>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&TimeSlicedPeerDataList> for LazyTimeSlicedPeerDataList {
    type Error = Error;
    fn try_from(val: &TimeSlicedPeerDataList) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyTimeSlicedPeerDataList> for TimeSlicedPeerDataList {
    type Error = Error;
    fn try_from(lazy: &LazyTimeSlicedPeerDataList) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
