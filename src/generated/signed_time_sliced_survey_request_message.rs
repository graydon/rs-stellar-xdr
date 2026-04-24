#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SignedTimeSlicedSurveyRequestMessage is an XDR Struct defined as:
///
/// ```text
/// struct SignedTimeSlicedSurveyRequestMessage
/// {
///     Signature requestSignature;
///     TimeSlicedSurveyRequestMessage request;
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
pub struct SignedTimeSlicedSurveyRequestMessage {
    pub request_signature: Signature,
    pub request: TimeSlicedSurveyRequestMessage,
}

impl ReadXdr for SignedTimeSlicedSurveyRequestMessage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                request_signature: Signature::read_xdr(r)?,
                request: TimeSlicedSurveyRequestMessage::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SignedTimeSlicedSurveyRequestMessage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.request_signature.write_xdr(w)?;
            self.request.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`SignedTimeSlicedSurveyRequestMessage`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySignedTimeSlicedSurveyRequestMessage(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazySignedTimeSlicedSurveyRequestMessage {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazySignedTimeSlicedSurveyRequestMessage {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.request_signature().cmp(&other.request_signature()))
            .then_with(|| self.request().cmp(&other.request()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazySignedTimeSlicedSurveyRequestMessage {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazySignature as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyTimeSlicedSurveyRequestMessage as LazyXdr>::xdr_validate(
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
        pos += <LazyTimeSlicedSurveyRequestMessage as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazySignedTimeSlicedSurveyRequestMessage {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazySignedTimeSlicedSurveyRequestMessage {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazySignedTimeSlicedSurveyRequestMessage {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazySignedTimeSlicedSurveyRequestMessage {
    /// Access field `request_signature`.
    #[must_use]
    pub fn request_signature(&self) -> LazySignature {
        <LazySignature as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `request`.
    #[must_use]
    pub fn request(&self) -> LazyTimeSlicedSurveyRequestMessage {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazySignature as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyTimeSlicedSurveyRequestMessage as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&SignedTimeSlicedSurveyRequestMessage> for LazySignedTimeSlicedSurveyRequestMessage {
    type Error = Error;
    fn try_from(val: &SignedTimeSlicedSurveyRequestMessage) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazySignedTimeSlicedSurveyRequestMessage> for SignedTimeSlicedSurveyRequestMessage {
    type Error = Error;
    fn try_from(lazy: &LazySignedTimeSlicedSurveyRequestMessage) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
