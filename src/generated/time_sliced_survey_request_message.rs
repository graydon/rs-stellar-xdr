#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// TimeSlicedSurveyRequestMessage is an XDR Struct defined as:
///
/// ```text
/// struct TimeSlicedSurveyRequestMessage
/// {
///     SurveyRequestMessage request;
///     uint32 nonce;
///     uint32 inboundPeersIndex;
///     uint32 outboundPeersIndex;
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
pub struct TimeSlicedSurveyRequestMessage {
    pub request: SurveyRequestMessage,
    pub nonce: u32,
    pub inbound_peers_index: u32,
    pub outbound_peers_index: u32,
}

impl ReadXdr for TimeSlicedSurveyRequestMessage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                request: SurveyRequestMessage::read_xdr(r)?,
                nonce: u32::read_xdr(r)?,
                inbound_peers_index: u32::read_xdr(r)?,
                outbound_peers_index: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TimeSlicedSurveyRequestMessage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.request.write_xdr(w)?;
            self.nonce.write_xdr(w)?;
            self.inbound_peers_index.write_xdr(w)?;
            self.outbound_peers_index.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`TimeSlicedSurveyRequestMessage`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyTimeSlicedSurveyRequestMessage(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyTimeSlicedSurveyRequestMessage {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyTimeSlicedSurveyRequestMessage {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.request().cmp(&other.request()))
            .then_with(|| self.nonce().cmp(&other.nonce()))
            .then_with(|| self.inbound_peers_index().cmp(&other.inbound_peers_index()))
            .then_with(|| {
                self.outbound_peers_index()
                    .cmp(&other.outbound_peers_index())
            })
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyTimeSlicedSurveyRequestMessage {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazySurveyRequestMessage as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(12).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazySurveyRequestMessage as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 12;
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyTimeSlicedSurveyRequestMessage {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyTimeSlicedSurveyRequestMessage {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyTimeSlicedSurveyRequestMessage {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyTimeSlicedSurveyRequestMessage {
    /// Access field `request`.
    #[must_use]
    pub fn request(&self) -> LazySurveyRequestMessage {
        <LazySurveyRequestMessage as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `nonce`.
    #[must_use]
    pub fn nonce(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazySurveyRequestMessage as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `inbound_peers_index`.
    #[must_use]
    pub fn inbound_peers_index(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazySurveyRequestMessage as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `outbound_peers_index`.
    #[must_use]
    pub fn outbound_peers_index(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazySurveyRequestMessage as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&TimeSlicedSurveyRequestMessage> for LazyTimeSlicedSurveyRequestMessage {
    type Error = Error;
    fn try_from(val: &TimeSlicedSurveyRequestMessage) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyTimeSlicedSurveyRequestMessage> for TimeSlicedSurveyRequestMessage {
    type Error = Error;
    fn try_from(lazy: &LazyTimeSlicedSurveyRequestMessage) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
