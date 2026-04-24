#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SurveyResponseMessage is an XDR Struct defined as:
///
/// ```text
/// struct SurveyResponseMessage
/// {
///     NodeID surveyorPeerID;
///     NodeID surveyedPeerID;
///     uint32 ledgerNum;
///     SurveyMessageCommandType commandType;
///     EncryptedBody encryptedBody;
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
pub struct SurveyResponseMessage {
    pub surveyor_peer_id: NodeId,
    pub surveyed_peer_id: NodeId,
    pub ledger_num: u32,
    pub command_type: SurveyMessageCommandType,
    pub encrypted_body: EncryptedBody,
}

impl ReadXdr for SurveyResponseMessage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                surveyor_peer_id: NodeId::read_xdr(r)?,
                surveyed_peer_id: NodeId::read_xdr(r)?,
                ledger_num: u32::read_xdr(r)?,
                command_type: SurveyMessageCommandType::read_xdr(r)?,
                encrypted_body: EncryptedBody::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SurveyResponseMessage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.surveyor_peer_id.write_xdr(w)?;
            self.surveyed_peer_id.write_xdr(w)?;
            self.ledger_num.write_xdr(w)?;
            self.command_type.write_xdr(w)?;
            self.encrypted_body.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`SurveyResponseMessage`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySurveyResponseMessage(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazySurveyResponseMessage {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazySurveyResponseMessage {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.surveyor_peer_id().cmp(&other.surveyor_peer_id()))
            .then_with(|| self.surveyed_peer_id().cmp(&other.surveyed_peer_id()))
            .then_with(|| self.ledger_num().cmp(&other.ledger_num()))
            .then_with(|| self.command_type().cmp(&other.command_type()))
            .then_with(|| self.encrypted_body().cmp(&other.encrypted_body()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazySurveyResponseMessage {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazyNodeId as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyNodeId as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(8).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <SurveyMessageCommandType as LazyXdr>::xdr_validate(&buf[(pos + 4) as usize..], depth)?;
        pos = next_pos;
        {
            let field_len =
                <LazyEncryptedBody as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        pos += <LazyEncryptedBody as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazySurveyResponseMessage {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazySurveyResponseMessage {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazySurveyResponseMessage {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazySurveyResponseMessage {
    /// Access field `surveyor_peer_id`.
    #[must_use]
    pub fn surveyor_peer_id(&self) -> LazyNodeId {
        <LazyNodeId as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `surveyed_peer_id`.
    #[must_use]
    pub fn surveyed_peer_id(&self) -> LazyNodeId {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyNodeId as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `ledger_num`.
    #[must_use]
    pub fn ledger_num(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `command_type`.
    #[must_use]
    pub fn command_type(&self) -> SurveyMessageCommandType {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        <SurveyMessageCommandType as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `encrypted_body`.
    #[must_use]
    pub fn encrypted_body(&self) -> LazyEncryptedBody {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        <LazyEncryptedBody as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&SurveyResponseMessage> for LazySurveyResponseMessage {
    type Error = Error;
    fn try_from(val: &SurveyResponseMessage) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazySurveyResponseMessage> for SurveyResponseMessage {
    type Error = Error;
    fn try_from(lazy: &LazySurveyResponseMessage) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
