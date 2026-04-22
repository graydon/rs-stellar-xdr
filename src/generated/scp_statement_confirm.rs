#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ScpStatementConfirm is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///         {
///             SCPBallot ballot;   // b
///             uint32 nPrepared;   // p.n
///             uint32 nCommit;     // c.n
///             uint32 nH;          // h.n
///             Hash quorumSetHash; // D
///         }
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
pub struct ScpStatementConfirm {
    pub ballot: ScpBallot,
    pub n_prepared: u32,
    pub n_commit: u32,
    pub n_h: u32,
    pub quorum_set_hash: Hash,
}

impl ReadXdr for ScpStatementConfirm {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ballot: ScpBallot::read_xdr(r)?,
                n_prepared: u32::read_xdr(r)?,
                n_commit: u32::read_xdr(r)?,
                n_h: u32::read_xdr(r)?,
                quorum_set_hash: Hash::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScpStatementConfirm {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ballot.write_xdr(w)?;
            self.n_prepared.write_xdr(w)?;
            self.n_commit.write_xdr(w)?;
            self.n_h.write_xdr(w)?;
            self.quorum_set_hash.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ScpStatementConfirm`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyScpStatementConfirm(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyScpStatementConfirm {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyScpStatementConfirm {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.ballot().cmp(&other.ballot()))
            .then_with(|| self.n_prepared().cmp(&other.n_prepared()))
            .then_with(|| self.n_commit().cmp(&other.n_commit()))
            .then_with(|| self.n_h().cmp(&other.n_h()))
            .then_with(|| self.quorum_set_hash().cmp(&other.quorum_set_hash()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyScpStatementConfirm {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazyScpBallot as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(44).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyHash as LazyXdr>::xdr_validate(&buf[(pos + 12) as usize..], depth)?;
        pos = next_pos;
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyScpBallot as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 44;
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyScpStatementConfirm {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyScpStatementConfirm {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyScpStatementConfirm {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyScpStatementConfirm {
    /// Access field `ballot`.
    #[must_use]
    pub fn ballot(&self) -> LazyScpBallot {
        <LazyScpBallot as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `n_prepared`.
    #[must_use]
    pub fn n_prepared(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyScpBallot as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `n_commit`.
    #[must_use]
    pub fn n_commit(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyScpBallot as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `n_h`.
    #[must_use]
    pub fn n_h(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyScpBallot as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `quorum_set_hash`.
    #[must_use]
    pub fn quorum_set_hash(&self) -> LazyHash {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyScpBallot as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 12;
        <LazyHash as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ScpStatementConfirm> for LazyScpStatementConfirm {
    type Error = Error;
    fn try_from(val: &ScpStatementConfirm) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyScpStatementConfirm> for ScpStatementConfirm {
    type Error = Error;
    fn try_from(lazy: &LazyScpStatementConfirm) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
