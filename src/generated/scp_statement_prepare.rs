#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ScpStatementPrepare is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///         {
///             Hash quorumSetHash;       // D
///             SCPBallot ballot;         // b
///             SCPBallot* prepared;      // p
///             SCPBallot* preparedPrime; // p'
///             uint32 nC;                // c.n
///             uint32 nH;                // h.n
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
pub struct ScpStatementPrepare {
    pub quorum_set_hash: Hash,
    pub ballot: ScpBallot,
    pub prepared: Option<ScpBallot>,
    pub prepared_prime: Option<ScpBallot>,
    pub n_c: u32,
    pub n_h: u32,
}

impl ReadXdr for ScpStatementPrepare {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                quorum_set_hash: Hash::read_xdr(r)?,
                ballot: ScpBallot::read_xdr(r)?,
                prepared: Option::<ScpBallot>::read_xdr(r)?,
                prepared_prime: Option::<ScpBallot>::read_xdr(r)?,
                n_c: u32::read_xdr(r)?,
                n_h: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScpStatementPrepare {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.quorum_set_hash.write_xdr(w)?;
            self.ballot.write_xdr(w)?;
            self.prepared.write_xdr(w)?;
            self.prepared_prime.write_xdr(w)?;
            self.n_c.write_xdr(w)?;
            self.n_h.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ScpStatementPrepare`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyScpStatementPrepare(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyScpStatementPrepare {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyScpStatementPrepare {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.quorum_set_hash().cmp(&other.quorum_set_hash()))
            .then_with(|| self.ballot().cmp(&other.ballot()))
            .then_with(|| self.prepared().cmp(&other.prepared()))
            .then_with(|| self.prepared_prime().cmp(&other.prepared_prime()))
            .then_with(|| self.n_c().cmp(&other.n_c()))
            .then_with(|| self.n_h().cmp(&other.n_h()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyScpStatementPrepare {
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
            let field_len = <LazyScpBallot as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyOption<LazyScpBallot> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyOption<LazyScpBallot> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(8).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += 32;
        pos += <LazyScpBallot as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazyScpBallot> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazyScpBallot> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
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
impl From<LazyHandle> for LazyScpStatementPrepare {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyScpStatementPrepare {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyScpStatementPrepare {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyScpStatementPrepare {
    /// Access field `quorum_set_hash`.
    #[must_use]
    pub fn quorum_set_hash(&self) -> LazyHash {
        <LazyHash as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `ballot`.
    #[must_use]
    pub fn ballot(&self) -> LazyScpBallot {
        <LazyScpBallot as LazyXdr>::from_xdr_at(&self.0, 32)
    }
    /// Access field `prepared`.
    #[must_use]
    pub fn prepared(&self) -> LazyOption<LazyScpBallot> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 32;
        pos += <LazyScpBallot as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyOption<LazyScpBallot> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `prepared_prime`.
    #[must_use]
    pub fn prepared_prime(&self) -> LazyOption<LazyScpBallot> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 32;
        pos += <LazyScpBallot as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazyScpBallot> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyOption<LazyScpBallot> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `n_c`.
    #[must_use]
    pub fn n_c(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 32;
        pos += <LazyScpBallot as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazyScpBallot> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazyScpBallot> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `n_h`.
    #[must_use]
    pub fn n_h(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 32;
        pos += <LazyScpBallot as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazyScpBallot> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazyScpBallot> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ScpStatementPrepare> for LazyScpStatementPrepare {
    type Error = Error;
    fn try_from(val: &ScpStatementPrepare) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyScpStatementPrepare> for ScpStatementPrepare {
    type Error = Error;
    fn try_from(lazy: &LazyScpStatementPrepare) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
