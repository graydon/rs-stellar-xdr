#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ScpStatement is an XDR Struct defined as:
///
/// ```text
/// struct SCPStatement
/// {
///     NodeID nodeID;    // v
///     uint64 slotIndex; // i
///
///     union switch (SCPStatementType type)
///     {
///     case SCP_ST_PREPARE:
///         struct
///         {
///             Hash quorumSetHash;       // D
///             SCPBallot ballot;         // b
///             SCPBallot* prepared;      // p
///             SCPBallot* preparedPrime; // p'
///             uint32 nC;                // c.n
///             uint32 nH;                // h.n
///         } prepare;
///     case SCP_ST_CONFIRM:
///         struct
///         {
///             SCPBallot ballot;   // b
///             uint32 nPrepared;   // p.n
///             uint32 nCommit;     // c.n
///             uint32 nH;          // h.n
///             Hash quorumSetHash; // D
///         } confirm;
///     case SCP_ST_EXTERNALIZE:
///         struct
///         {
///             SCPBallot commit;         // c
///             uint32 nH;                // h.n
///             Hash commitQuorumSetHash; // D used before EXTERNALIZE
///         } externalize;
///     case SCP_ST_NOMINATE:
///         SCPNomination nominate;
///     }
///     pledges;
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
pub struct ScpStatement {
    pub node_id: NodeId,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub slot_index: u64,
    pub pledges: ScpStatementPledges,
}

impl ReadXdr for ScpStatement {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                node_id: NodeId::read_xdr(r)?,
                slot_index: u64::read_xdr(r)?,
                pledges: ScpStatementPledges::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScpStatement {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.node_id.write_xdr(w)?;
            self.slot_index.write_xdr(w)?;
            self.pledges.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ScpStatement`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyScpStatement(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyScpStatement {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyScpStatement {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.node_id().cmp(&other.node_id()))
            .then_with(|| self.slot_index().cmp(&other.slot_index()))
            .then_with(|| self.pledges().cmp(&other.pledges()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyScpStatement {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazyNodeId as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(8).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        {
            let field_len =
                <LazyScpStatementPledges as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        pos += <LazyScpStatementPledges as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyScpStatement {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyScpStatement {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyScpStatement {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyScpStatement {
    /// Access field `node_id`.
    #[must_use]
    pub fn node_id(&self) -> LazyNodeId {
        <LazyNodeId as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `slot_index`.
    #[must_use]
    pub fn slot_index(&self) -> u64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <u64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `pledges`.
    #[must_use]
    pub fn pledges(&self) -> LazyScpStatementPledges {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyNodeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        <LazyScpStatementPledges as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ScpStatement> for LazyScpStatement {
    type Error = Error;
    fn try_from(val: &ScpStatement) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyScpStatement> for ScpStatement {
    type Error = Error;
    fn try_from(lazy: &LazyScpStatement) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
