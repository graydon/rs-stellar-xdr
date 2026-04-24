#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ScpNomination is an XDR Struct defined as:
///
/// ```text
/// struct SCPNomination
/// {
///     Hash quorumSetHash; // D
///     Value votes<>;      // X
///     Value accepted<>;   // Y
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
pub struct ScpNomination {
    pub quorum_set_hash: Hash,
    pub votes: VecM<Value>,
    pub accepted: VecM<Value>,
}

impl ReadXdr for ScpNomination {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                quorum_set_hash: Hash::read_xdr(r)?,
                votes: VecM::<Value>::read_xdr(r)?,
                accepted: VecM::<Value>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScpNomination {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.quorum_set_hash.write_xdr(w)?;
            self.votes.write_xdr(w)?;
            self.accepted.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ScpNomination`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyScpNomination(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyScpNomination {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyScpNomination {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.quorum_set_hash().cmp(&other.quorum_set_hash()))
            .then_with(|| self.votes().cmp(&other.votes()))
            .then_with(|| self.accepted().cmp(&other.accepted()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyScpNomination {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(32).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyHash as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        {
            let field_len =
                <LazyVecM<LazyValue> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyVecM<LazyValue> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += 32;
        pos += <LazyVecM<LazyValue> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyValue> as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyScpNomination {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyScpNomination {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyScpNomination {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyScpNomination {
    /// Access field `quorum_set_hash`.
    #[must_use]
    pub fn quorum_set_hash(&self) -> LazyHash {
        <LazyHash as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `votes`.
    #[must_use]
    pub fn votes(&self) -> LazyVecM<LazyValue> {
        <LazyVecM<LazyValue> as LazyXdr>::from_xdr_at(&self.0, 32)
    }
    /// Access field `accepted`.
    #[must_use]
    pub fn accepted(&self) -> LazyVecM<LazyValue> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 32;
        pos += <LazyVecM<LazyValue> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyValue> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ScpNomination> for LazyScpNomination {
    type Error = Error;
    fn try_from(val: &ScpNomination) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyScpNomination> for ScpNomination {
    type Error = Error;
    fn try_from(lazy: &LazyScpNomination) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
