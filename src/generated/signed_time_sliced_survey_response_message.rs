#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SignedTimeSlicedSurveyResponseMessage is an XDR Struct defined as:
///
/// ```text
/// struct SignedTimeSlicedSurveyResponseMessage
/// {
///     Signature responseSignature;
///     TimeSlicedSurveyResponseMessage response;
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
pub struct SignedTimeSlicedSurveyResponseMessage {
    pub response_signature: Signature,
    pub response: TimeSlicedSurveyResponseMessage,
}

impl ReadXdr for SignedTimeSlicedSurveyResponseMessage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                response_signature: Signature::read_xdr(r)?,
                response: TimeSlicedSurveyResponseMessage::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SignedTimeSlicedSurveyResponseMessage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.response_signature.write_xdr(w)?;
            self.response.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`SignedTimeSlicedSurveyResponseMessage`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySignedTimeSlicedSurveyResponseMessage(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazySignedTimeSlicedSurveyResponseMessage {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazySignedTimeSlicedSurveyResponseMessage {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.response_signature().cmp(&other.response_signature()))
            .then_with(|| self.response().cmp(&other.response()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazySignedTimeSlicedSurveyResponseMessage {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazySignature as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyTimeSlicedSurveyResponseMessage as LazyXdr>::xdr_validate(
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
        pos += <LazyTimeSlicedSurveyResponseMessage as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazySignedTimeSlicedSurveyResponseMessage {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazySignedTimeSlicedSurveyResponseMessage {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazySignedTimeSlicedSurveyResponseMessage {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazySignedTimeSlicedSurveyResponseMessage {
    /// Access field `response_signature`.
    #[must_use]
    pub fn response_signature(&self) -> LazySignature {
        <LazySignature as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `response`.
    #[must_use]
    pub fn response(&self) -> LazyTimeSlicedSurveyResponseMessage {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazySignature as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyTimeSlicedSurveyResponseMessage as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&SignedTimeSlicedSurveyResponseMessage> for LazySignedTimeSlicedSurveyResponseMessage {
    type Error = Error;
    fn try_from(val: &SignedTimeSlicedSurveyResponseMessage) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazySignedTimeSlicedSurveyResponseMessage> for SignedTimeSlicedSurveyResponseMessage {
    type Error = Error;
    fn try_from(lazy: &LazySignedTimeSlicedSurveyResponseMessage) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
