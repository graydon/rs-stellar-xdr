#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SurveyRequestMessage is an XDR Struct defined as:
///
/// ```text
/// struct SurveyRequestMessage
/// {
///     NodeID surveyorPeerID;
///     NodeID surveyedPeerID;
///     uint32 ledgerNum;
///     Curve25519Public encryptionKey;
///     SurveyMessageCommandType commandType;
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
pub struct SurveyRequestMessage {
    pub surveyor_peer_id: NodeId,
    pub surveyed_peer_id: NodeId,
    pub ledger_num: u32,
    pub encryption_key: Curve25519Public,
    pub command_type: SurveyMessageCommandType,
}

impl ReadXdr for SurveyRequestMessage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                surveyor_peer_id: NodeId::read_xdr(r)?,
                surveyed_peer_id: NodeId::read_xdr(r)?,
                ledger_num: u32::read_xdr(r)?,
                encryption_key: Curve25519Public::read_xdr(r)?,
                command_type: SurveyMessageCommandType::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SurveyRequestMessage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.surveyor_peer_id.write_xdr(w)?;
            self.surveyed_peer_id.write_xdr(w)?;
            self.ledger_num.write_xdr(w)?;
            self.encryption_key.write_xdr(w)?;
            self.command_type.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`SurveyRequestMessage`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySurveyRequestMessage(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazySurveyRequestMessage {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazySurveyRequestMessage {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.surveyor_peer_id().cmp(&other.surveyor_peer_id()))
            .then_with(|| self.surveyed_peer_id().cmp(&other.surveyed_peer_id()))
            .then_with(|| self.ledger_num().cmp(&other.ledger_num()))
            .then_with(|| self.encryption_key().cmp(&other.encryption_key()))
            .then_with(|| self.command_type().cmp(&other.command_type()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazySurveyRequestMessage {
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
        let next_pos = pos.checked_add(40).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyCurve25519Public as LazyXdr>::xdr_validate(&buf[(pos + 4) as usize..], depth)?;
        <SurveyMessageCommandType as LazyXdr>::xdr_validate(&buf[(pos + 36) as usize..], depth)?;
        pos = next_pos;
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 40;
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
impl From<LazyHandle> for LazySurveyRequestMessage {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazySurveyRequestMessage {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazySurveyRequestMessage {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazySurveyRequestMessage {
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
    /// Access field `encryption_key`.
    #[must_use]
    pub fn encryption_key(&self) -> LazyCurve25519Public {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        <LazyCurve25519Public as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `command_type`.
    #[must_use]
    pub fn command_type(&self) -> SurveyMessageCommandType {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 36;
        <SurveyMessageCommandType as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&SurveyRequestMessage> for LazySurveyRequestMessage {
    type Error = Error;
    fn try_from(val: &SurveyRequestMessage) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazySurveyRequestMessage> for SurveyRequestMessage {
    type Error = Error;
    fn try_from(lazy: &LazySurveyRequestMessage) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
