#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SignedTimeSlicedSurveyStopCollectingMessage is an XDR Struct defined as:
///
/// ```text
/// struct SignedTimeSlicedSurveyStopCollectingMessage
/// {
///     Signature signature;
///     TimeSlicedSurveyStopCollectingMessage stopCollecting;
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
pub struct SignedTimeSlicedSurveyStopCollectingMessage {
    pub signature: Signature,
    pub stop_collecting: TimeSlicedSurveyStopCollectingMessage,
}

impl ReadXdr for SignedTimeSlicedSurveyStopCollectingMessage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                signature: Signature::read_xdr(r)?,
                stop_collecting: TimeSlicedSurveyStopCollectingMessage::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SignedTimeSlicedSurveyStopCollectingMessage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.signature.write_xdr(w)?;
            self.stop_collecting.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`SignedTimeSlicedSurveyStopCollectingMessage`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySignedTimeSlicedSurveyStopCollectingMessage(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazySignedTimeSlicedSurveyStopCollectingMessage {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazySignedTimeSlicedSurveyStopCollectingMessage {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.signature().cmp(&other.signature()))
            .then_with(|| self.stop_collecting().cmp(&other.stop_collecting()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazySignedTimeSlicedSurveyStopCollectingMessage {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazySignature as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyTimeSlicedSurveyStopCollectingMessage as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazySignature as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos +=
            <LazyTimeSlicedSurveyStopCollectingMessage as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazySignedTimeSlicedSurveyStopCollectingMessage {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazySignedTimeSlicedSurveyStopCollectingMessage {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazySignedTimeSlicedSurveyStopCollectingMessage {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazySignedTimeSlicedSurveyStopCollectingMessage {
    /// Access field `signature`.
    #[must_use]
    pub fn signature(&self) -> LazySignature {
        <LazySignature as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `stop_collecting`.
    #[must_use]
    pub fn stop_collecting(&self) -> LazyTimeSlicedSurveyStopCollectingMessage {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazySignature as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyTimeSlicedSurveyStopCollectingMessage as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&SignedTimeSlicedSurveyStopCollectingMessage>
    for LazySignedTimeSlicedSurveyStopCollectingMessage
{
    type Error = Error;
    fn try_from(val: &SignedTimeSlicedSurveyStopCollectingMessage) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazySignedTimeSlicedSurveyStopCollectingMessage>
    for SignedTimeSlicedSurveyStopCollectingMessage
{
    type Error = Error;
    fn try_from(lazy: &LazySignedTimeSlicedSurveyStopCollectingMessage) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
