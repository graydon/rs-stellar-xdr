#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ScpStatementExternalize is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///         {
///             SCPBallot commit;         // c
///             uint32 nH;                // h.n
///             Hash commitQuorumSetHash; // D used before EXTERNALIZE
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
pub struct ScpStatementExternalize {
    pub commit: ScpBallot,
    pub n_h: u32,
    pub commit_quorum_set_hash: Hash,
}

impl ReadXdr for ScpStatementExternalize {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                commit: ScpBallot::read_xdr(r)?,
                n_h: u32::read_xdr(r)?,
                commit_quorum_set_hash: Hash::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScpStatementExternalize {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.commit.write_xdr(w)?;
            self.n_h.write_xdr(w)?;
            self.commit_quorum_set_hash.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ScpStatementExternalize`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyScpStatementExternalize(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyScpStatementExternalize {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyScpStatementExternalize {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.commit().cmp(&other.commit()))
            .then_with(|| self.n_h().cmp(&other.n_h()))
            .then_with(|| {
                self.commit_quorum_set_hash()
                    .cmp(&other.commit_quorum_set_hash())
            })
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyScpStatementExternalize {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazyScpBallot as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(36).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyHash as LazyXdr>::xdr_validate(&buf[(pos + 4) as usize..], depth)?;
        pos = next_pos;
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyScpBallot as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 36;
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
impl From<LazyHandle> for LazyScpStatementExternalize {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyScpStatementExternalize {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyScpStatementExternalize {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyScpStatementExternalize {
    /// Access field `commit`.
    #[must_use]
    pub fn commit(&self) -> LazyScpBallot {
        <LazyScpBallot as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `n_h`.
    #[must_use]
    pub fn n_h(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyScpBallot as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `commit_quorum_set_hash`.
    #[must_use]
    pub fn commit_quorum_set_hash(&self) -> LazyHash {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyScpBallot as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        <LazyHash as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ScpStatementExternalize> for LazyScpStatementExternalize {
    type Error = Error;
    fn try_from(val: &ScpStatementExternalize) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyScpStatementExternalize> for ScpStatementExternalize {
    type Error = Error;
    fn try_from(lazy: &LazyScpStatementExternalize) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
