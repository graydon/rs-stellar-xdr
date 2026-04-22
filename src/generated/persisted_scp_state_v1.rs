#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// PersistedScpStateV1 is an XDR Struct defined as:
///
/// ```text
/// struct PersistedSCPStateV1
/// {
/// 	// Tx sets are saved separately
/// 	SCPEnvelope scpEnvelopes<>;
/// 	SCPQuorumSet quorumSets<>;
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
pub struct PersistedScpStateV1 {
    pub scp_envelopes: VecM<ScpEnvelope>,
    pub quorum_sets: VecM<ScpQuorumSet>,
}

impl ReadXdr for PersistedScpStateV1 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                scp_envelopes: VecM::<ScpEnvelope>::read_xdr(r)?,
                quorum_sets: VecM::<ScpQuorumSet>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for PersistedScpStateV1 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.scp_envelopes.write_xdr(w)?;
            self.quorum_sets.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`PersistedScpStateV1`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyPersistedScpStateV1(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyPersistedScpStateV1 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyPersistedScpStateV1 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.scp_envelopes().cmp(&other.scp_envelopes()))
            .then_with(|| self.quorum_sets().cmp(&other.quorum_sets()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyPersistedScpStateV1 {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyVecM<LazyScpEnvelope> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyVecM<LazyScpQuorumSet> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyVecM<LazyScpEnvelope> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyScpQuorumSet> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyPersistedScpStateV1 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyPersistedScpStateV1 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyPersistedScpStateV1 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyPersistedScpStateV1 {
    /// Access field `scp_envelopes`.
    #[must_use]
    pub fn scp_envelopes(&self) -> LazyVecM<LazyScpEnvelope> {
        <LazyVecM<LazyScpEnvelope> as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `quorum_sets`.
    #[must_use]
    pub fn quorum_sets(&self) -> LazyVecM<LazyScpQuorumSet> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyVecM<LazyScpEnvelope> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyScpQuorumSet> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&PersistedScpStateV1> for LazyPersistedScpStateV1 {
    type Error = Error;
    fn try_from(val: &PersistedScpStateV1) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyPersistedScpStateV1> for PersistedScpStateV1 {
    type Error = Error;
    fn try_from(lazy: &LazyPersistedScpStateV1) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
