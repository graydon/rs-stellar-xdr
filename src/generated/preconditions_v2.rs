#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// PreconditionsV2 is an XDR Struct defined as:
///
/// ```text
/// struct PreconditionsV2
/// {
///     TimeBounds* timeBounds;
///
///     // Transaction only valid for ledger numbers n such that
///     // minLedger <= n < maxLedger (if maxLedger == 0, then
///     // only minLedger is checked)
///     LedgerBounds* ledgerBounds;
///
///     // If NULL, only valid when sourceAccount's sequence number
///     // is seqNum - 1.  Otherwise, valid when sourceAccount's
///     // sequence number n satisfies minSeqNum <= n < tx.seqNum.
///     // Note that after execution the account's sequence number
///     // is always raised to tx.seqNum, and a transaction is not
///     // valid if tx.seqNum is too high to ensure replay protection.
///     SequenceNumber* minSeqNum;
///
///     // For the transaction to be valid, the current ledger time must
///     // be at least minSeqAge greater than sourceAccount's seqTime.
///     Duration minSeqAge;
///
///     // For the transaction to be valid, the current ledger number
///     // must be at least minSeqLedgerGap greater than sourceAccount's
///     // seqLedger.
///     uint32 minSeqLedgerGap;
///
///     // For the transaction to be valid, there must be a signature
///     // corresponding to every Signer in this array, even if the
///     // signature is not otherwise required by the sourceAccount or
///     // operations.
///     SignerKey extraSigners<2>;
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
pub struct PreconditionsV2 {
    pub time_bounds: Option<TimeBounds>,
    pub ledger_bounds: Option<LedgerBounds>,
    pub min_seq_num: Option<SequenceNumber>,
    pub min_seq_age: Duration,
    pub min_seq_ledger_gap: u32,
    pub extra_signers: VecM<SignerKey, 2>,
}

impl ReadXdr for PreconditionsV2 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                time_bounds: Option::<TimeBounds>::read_xdr(r)?,
                ledger_bounds: Option::<LedgerBounds>::read_xdr(r)?,
                min_seq_num: Option::<SequenceNumber>::read_xdr(r)?,
                min_seq_age: Duration::read_xdr(r)?,
                min_seq_ledger_gap: u32::read_xdr(r)?,
                extra_signers: VecM::<SignerKey, 2>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for PreconditionsV2 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.time_bounds.write_xdr(w)?;
            self.ledger_bounds.write_xdr(w)?;
            self.min_seq_num.write_xdr(w)?;
            self.min_seq_age.write_xdr(w)?;
            self.min_seq_ledger_gap.write_xdr(w)?;
            self.extra_signers.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`PreconditionsV2`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyPreconditionsV2(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyPreconditionsV2 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyPreconditionsV2 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.time_bounds().cmp(&other.time_bounds()))
            .then_with(|| self.ledger_bounds().cmp(&other.ledger_bounds()))
            .then_with(|| self.min_seq_num().cmp(&other.min_seq_num()))
            .then_with(|| self.min_seq_age().cmp(&other.min_seq_age()))
            .then_with(|| self.min_seq_ledger_gap().cmp(&other.min_seq_ledger_gap()))
            .then_with(|| self.extra_signers().cmp(&other.extra_signers()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyPreconditionsV2 {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyOption<LazyTimeBounds> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyOption<LazyLedgerBounds> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyOption<LazySequenceNumber> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(12).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        {
            let field_len =
                <LazyVecM<LazySignerKey, 2> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyOption<LazyTimeBounds> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazyLedgerBounds> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazySequenceNumber> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 12;
        pos += <LazyVecM<LazySignerKey, 2> as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyPreconditionsV2 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyPreconditionsV2 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyPreconditionsV2 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyPreconditionsV2 {
    /// Access field `time_bounds`.
    #[must_use]
    pub fn time_bounds(&self) -> LazyOption<LazyTimeBounds> {
        <LazyOption<LazyTimeBounds> as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `ledger_bounds`.
    #[must_use]
    pub fn ledger_bounds(&self) -> LazyOption<LazyLedgerBounds> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyOption<LazyTimeBounds> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyOption<LazyLedgerBounds> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `min_seq_num`.
    #[must_use]
    pub fn min_seq_num(&self) -> LazyOption<LazySequenceNumber> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyOption<LazyTimeBounds> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazyLedgerBounds> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyOption<LazySequenceNumber> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `min_seq_age`.
    #[must_use]
    pub fn min_seq_age(&self) -> LazyDuration {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyOption<LazyTimeBounds> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazyLedgerBounds> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazySequenceNumber> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyDuration as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `min_seq_ledger_gap`.
    #[must_use]
    pub fn min_seq_ledger_gap(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyOption<LazyTimeBounds> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazyLedgerBounds> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazySequenceNumber> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `extra_signers`.
    #[must_use]
    pub fn extra_signers(&self) -> LazyVecM<LazySignerKey, 2> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyOption<LazyTimeBounds> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazyLedgerBounds> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazySequenceNumber> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 12;
        <LazyVecM<LazySignerKey, 2> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&PreconditionsV2> for LazyPreconditionsV2 {
    type Error = Error;
    fn try_from(val: &PreconditionsV2) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyPreconditionsV2> for PreconditionsV2 {
    type Error = Error;
    fn try_from(lazy: &LazyPreconditionsV2) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
