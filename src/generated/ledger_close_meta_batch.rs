#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// LedgerCloseMetaBatch is an XDR Struct defined as:
///
/// ```text
/// struct LedgerCloseMetaBatch
/// {
///     // starting ledger sequence number in the batch
///     uint32 startSequence;
///
///     // ending ledger sequence number in the batch
///     uint32 endSequence;
///
///     // Ledger close meta for each ledger within the batch
///     LedgerCloseMeta ledgerCloseMetas<>;
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
pub struct LedgerCloseMetaBatch {
    pub start_sequence: u32,
    pub end_sequence: u32,
    pub ledger_close_metas: VecM<LedgerCloseMeta>,
}

impl ReadXdr for LedgerCloseMetaBatch {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                start_sequence: u32::read_xdr(r)?,
                end_sequence: u32::read_xdr(r)?,
                ledger_close_metas: VecM::<LedgerCloseMeta>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerCloseMetaBatch {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.start_sequence.write_xdr(w)?;
            self.end_sequence.write_xdr(w)?;
            self.ledger_close_metas.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`LedgerCloseMetaBatch`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyLedgerCloseMetaBatch(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyLedgerCloseMetaBatch {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyLedgerCloseMetaBatch {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.start_sequence().cmp(&other.start_sequence()))
            .then_with(|| self.end_sequence().cmp(&other.end_sequence()))
            .then_with(|| self.ledger_close_metas().cmp(&other.ledger_close_metas()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyLedgerCloseMetaBatch {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(8).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        {
            let field_len = <LazyVecM<LazyLedgerCloseMeta> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += 8;
        pos += <LazyVecM<LazyLedgerCloseMeta> as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyLedgerCloseMetaBatch {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyLedgerCloseMetaBatch {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyLedgerCloseMetaBatch {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyLedgerCloseMetaBatch {
    /// Access field `start_sequence`.
    #[must_use]
    pub fn start_sequence(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `end_sequence`.
    #[must_use]
    pub fn end_sequence(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 4)
    }
    /// Access field `ledger_close_metas`.
    #[must_use]
    pub fn ledger_close_metas(&self) -> LazyVecM<LazyLedgerCloseMeta> {
        <LazyVecM<LazyLedgerCloseMeta> as LazyXdr>::from_xdr_at(&self.0, 8)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LedgerCloseMetaBatch> for LazyLedgerCloseMetaBatch {
    type Error = Error;
    fn try_from(val: &LedgerCloseMetaBatch) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyLedgerCloseMetaBatch> for LedgerCloseMetaBatch {
    type Error = Error;
    fn try_from(lazy: &LazyLedgerCloseMetaBatch) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
