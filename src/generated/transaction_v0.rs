#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// TransactionV0 is an XDR Struct defined as:
///
/// ```text
/// struct TransactionV0
/// {
///     uint256 sourceAccountEd25519;
///     uint32 fee;
///     SequenceNumber seqNum;
///     TimeBounds* timeBounds;
///     Memo memo;
///     Operation operations<MAX_OPS_PER_TX>;
///     union switch (int v)
///     {
///     case 0:
///         void;
///     }
///     ext;
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
pub struct TransactionV0 {
    pub source_account_ed25519: Uint256,
    pub fee: u32,
    pub seq_num: SequenceNumber,
    pub time_bounds: Option<TimeBounds>,
    pub memo: Memo,
    pub operations: VecM<Operation, 100>,
    pub ext: TransactionV0Ext,
}

impl ReadXdr for TransactionV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                source_account_ed25519: Uint256::read_xdr(r)?,
                fee: u32::read_xdr(r)?,
                seq_num: SequenceNumber::read_xdr(r)?,
                time_bounds: Option::<TimeBounds>::read_xdr(r)?,
                memo: Memo::read_xdr(r)?,
                operations: VecM::<Operation, 100>::read_xdr(r)?,
                ext: TransactionV0Ext::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.source_account_ed25519.write_xdr(w)?;
            self.fee.write_xdr(w)?;
            self.seq_num.write_xdr(w)?;
            self.time_bounds.write_xdr(w)?;
            self.memo.write_xdr(w)?;
            self.operations.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`TransactionV0`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyTransactionV0(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyTransactionV0 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyTransactionV0 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| {
                self.source_account_ed25519()
                    .cmp(&other.source_account_ed25519())
            })
            .then_with(|| self.fee().cmp(&other.fee()))
            .then_with(|| self.seq_num().cmp(&other.seq_num()))
            .then_with(|| self.time_bounds().cmp(&other.time_bounds()))
            .then_with(|| self.memo().cmp(&other.memo()))
            .then_with(|| self.operations().cmp(&other.operations()))
            .then_with(|| self.ext().cmp(&other.ext()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyTransactionV0 {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(44).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyUint256 as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        {
            let field_len =
                <LazyOption<LazyTimeBounds> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyMemo as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyVecM<LazyOperation, 100> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyTransactionV0Ext as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += 44;
        pos += <LazyOption<LazyTimeBounds> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyMemo as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyOperation, 100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyTransactionV0Ext as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyTransactionV0 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyTransactionV0 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyTransactionV0 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyTransactionV0 {
    /// Access field `source_account_ed25519`.
    #[must_use]
    pub fn source_account_ed25519(&self) -> LazyUint256 {
        <LazyUint256 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `fee`.
    #[must_use]
    pub fn fee(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 32)
    }
    /// Access field `seq_num`.
    #[must_use]
    pub fn seq_num(&self) -> LazySequenceNumber {
        <LazySequenceNumber as LazyXdr>::from_xdr_at(&self.0, 36)
    }
    /// Access field `time_bounds`.
    #[must_use]
    pub fn time_bounds(&self) -> LazyOption<LazyTimeBounds> {
        <LazyOption<LazyTimeBounds> as LazyXdr>::from_xdr_at(&self.0, 44)
    }
    /// Access field `memo`.
    #[must_use]
    pub fn memo(&self) -> LazyMemo {
        let buf = self.0.as_slice();
        let mut pos: u32 = 44;
        pos += <LazyOption<LazyTimeBounds> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyMemo as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `operations`.
    #[must_use]
    pub fn operations(&self) -> LazyVecM<LazyOperation, 100> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 44;
        pos += <LazyOption<LazyTimeBounds> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyMemo as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyOperation, 100> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazyTransactionV0Ext {
        let buf = self.0.as_slice();
        let mut pos: u32 = 44;
        pos += <LazyOption<LazyTimeBounds> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyMemo as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyOperation, 100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyTransactionV0Ext as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&TransactionV0> for LazyTransactionV0 {
    type Error = Error;
    fn try_from(val: &TransactionV0) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyTransactionV0> for TransactionV0 {
    type Error = Error;
    fn try_from(lazy: &LazyTransactionV0) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
