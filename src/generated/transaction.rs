#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// Transaction is an XDR Struct defined as:
///
/// ```text
/// struct Transaction
/// {
///     // account used to run the transaction
///     MuxedAccount sourceAccount;
///
///     // the fee the sourceAccount will pay
///     uint32 fee;
///
///     // sequence number to consume in the account
///     SequenceNumber seqNum;
///
///     // validity conditions
///     Preconditions cond;
///
///     Memo memo;
///
///     Operation operations<MAX_OPS_PER_TX>;
///
///     union switch (int v)
///     {
///     case 0:
///         void;
///     case 1:
///         SorobanTransactionData sorobanData;
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
pub struct Transaction {
    pub source_account: MuxedAccount,
    pub fee: u32,
    pub seq_num: SequenceNumber,
    pub cond: Preconditions,
    pub memo: Memo,
    pub operations: VecM<Operation, 100>,
    pub ext: TransactionExt,
}

impl ReadXdr for Transaction {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                source_account: MuxedAccount::read_xdr(r)?,
                fee: u32::read_xdr(r)?,
                seq_num: SequenceNumber::read_xdr(r)?,
                cond: Preconditions::read_xdr(r)?,
                memo: Memo::read_xdr(r)?,
                operations: VecM::<Operation, 100>::read_xdr(r)?,
                ext: TransactionExt::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for Transaction {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.source_account.write_xdr(w)?;
            self.fee.write_xdr(w)?;
            self.seq_num.write_xdr(w)?;
            self.cond.write_xdr(w)?;
            self.memo.write_xdr(w)?;
            self.operations.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`Transaction`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyTransaction(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyTransaction {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyTransaction {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.source_account().cmp(&other.source_account()))
            .then_with(|| self.fee().cmp(&other.fee()))
            .then_with(|| self.seq_num().cmp(&other.seq_num()))
            .then_with(|| self.cond().cmp(&other.cond()))
            .then_with(|| self.memo().cmp(&other.memo()))
            .then_with(|| self.operations().cmp(&other.operations()))
            .then_with(|| self.ext().cmp(&other.ext()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyTransaction {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyMuxedAccount as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(12).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        {
            let field_len =
                <LazyPreconditions as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
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
                <LazyTransactionExt as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyMuxedAccount as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 12;
        pos += <LazyPreconditions as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyMemo as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyOperation, 100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyTransactionExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyTransaction {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyTransaction {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyTransaction {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyTransaction {
    /// Access field `source_account`.
    #[must_use]
    pub fn source_account(&self) -> LazyMuxedAccount {
        <LazyMuxedAccount as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `fee`.
    #[must_use]
    pub fn fee(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyMuxedAccount as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `seq_num`.
    #[must_use]
    pub fn seq_num(&self) -> LazySequenceNumber {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyMuxedAccount as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        <LazySequenceNumber as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `cond`.
    #[must_use]
    pub fn cond(&self) -> LazyPreconditions {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyMuxedAccount as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 12;
        <LazyPreconditions as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `memo`.
    #[must_use]
    pub fn memo(&self) -> LazyMemo {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyMuxedAccount as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 12;
        pos += <LazyPreconditions as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyMemo as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `operations`.
    #[must_use]
    pub fn operations(&self) -> LazyVecM<LazyOperation, 100> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyMuxedAccount as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 12;
        pos += <LazyPreconditions as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyMemo as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyOperation, 100> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazyTransactionExt {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyMuxedAccount as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 12;
        pos += <LazyPreconditions as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyMemo as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyOperation, 100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyTransactionExt as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&Transaction> for LazyTransaction {
    type Error = Error;
    fn try_from(val: &Transaction) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyTransaction> for Transaction {
    type Error = Error;
    fn try_from(lazy: &LazyTransaction) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
