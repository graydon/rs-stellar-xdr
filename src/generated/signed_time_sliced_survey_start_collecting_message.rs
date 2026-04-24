#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SignedTimeSlicedSurveyStartCollectingMessage is an XDR Struct defined as:
///
/// ```text
/// struct SignedTimeSlicedSurveyStartCollectingMessage
/// {
///     Signature signature;
///     TimeSlicedSurveyStartCollectingMessage startCollecting;
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
pub struct SignedTimeSlicedSurveyStartCollectingMessage {
    pub signature: Signature,
    pub start_collecting: TimeSlicedSurveyStartCollectingMessage,
}

impl ReadXdr for SignedTimeSlicedSurveyStartCollectingMessage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                signature: Signature::read_xdr(r)?,
                start_collecting: TimeSlicedSurveyStartCollectingMessage::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SignedTimeSlicedSurveyStartCollectingMessage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.signature.write_xdr(w)?;
            self.start_collecting.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`SignedTimeSlicedSurveyStartCollectingMessage`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySignedTimeSlicedSurveyStartCollectingMessage(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazySignedTimeSlicedSurveyStartCollectingMessage {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazySignedTimeSlicedSurveyStartCollectingMessage {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.signature().cmp(&other.signature()))
            .then_with(|| self.start_collecting().cmp(&other.start_collecting()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazySignedTimeSlicedSurveyStartCollectingMessage {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazySignature as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyTimeSlicedSurveyStartCollectingMessage as LazyXdr>::xdr_validate(
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
            <LazyTimeSlicedSurveyStartCollectingMessage as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos
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
impl From<LazyHandle> for LazySignedTimeSlicedSurveyStartCollectingMessage {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazySignedTimeSlicedSurveyStartCollectingMessage {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazySignedTimeSlicedSurveyStartCollectingMessage {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazySignedTimeSlicedSurveyStartCollectingMessage {
    /// Access field `signature`.
    #[must_use]
    pub fn signature(&self) -> LazySignature {
        <LazySignature as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `start_collecting`.
    #[must_use]
    pub fn start_collecting(&self) -> LazyTimeSlicedSurveyStartCollectingMessage {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazySignature as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyTimeSlicedSurveyStartCollectingMessage as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&SignedTimeSlicedSurveyStartCollectingMessage>
    for LazySignedTimeSlicedSurveyStartCollectingMessage
{
    type Error = Error;
    fn try_from(val: &SignedTimeSlicedSurveyStartCollectingMessage) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazySignedTimeSlicedSurveyStartCollectingMessage>
    for SignedTimeSlicedSurveyStartCollectingMessage
{
    type Error = Error;
    fn try_from(lazy: &LazySignedTimeSlicedSurveyStartCollectingMessage) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
