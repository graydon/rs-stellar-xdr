#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// InnerTransactionResult is an XDR Struct defined as:
///
/// ```text
/// struct InnerTransactionResult
/// {
///     // Always 0. Here for binary compatibility.
///     int64 feeCharged;
///
///     union switch (TransactionResultCode code)
///     {
///     // txFEE_BUMP_INNER_SUCCESS is not included
///     case txSUCCESS:
///     case txFAILED:
///         OperationResult results<>;
///     case txTOO_EARLY:
///     case txTOO_LATE:
///     case txMISSING_OPERATION:
///     case txBAD_SEQ:
///     case txBAD_AUTH:
///     case txINSUFFICIENT_BALANCE:
///     case txNO_ACCOUNT:
///     case txINSUFFICIENT_FEE:
///     case txBAD_AUTH_EXTRA:
///     case txINTERNAL_ERROR:
///     case txNOT_SUPPORTED:
///     // txFEE_BUMP_INNER_FAILED is not included
///     case txBAD_SPONSORSHIP:
///     case txBAD_MIN_SEQ_AGE_OR_GAP:
///     case txMALFORMED:
///     case txSOROBAN_INVALID:
///     case txFROZEN_KEY_ACCESSED:
///         void;
///     }
///     result;
///
///     // reserved for future use
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
pub struct InnerTransactionResult {
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub fee_charged: i64,
    pub result: InnerTransactionResultResult,
    pub ext: InnerTransactionResultExt,
}

impl ReadXdr for InnerTransactionResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                fee_charged: i64::read_xdr(r)?,
                result: InnerTransactionResultResult::read_xdr(r)?,
                ext: InnerTransactionResultExt::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for InnerTransactionResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.fee_charged.write_xdr(w)?;
            self.result.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`InnerTransactionResult`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyInnerTransactionResult(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyInnerTransactionResult {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyInnerTransactionResult {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.fee_charged().cmp(&other.fee_charged()))
            .then_with(|| self.result().cmp(&other.result()))
            .then_with(|| self.ext().cmp(&other.ext()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyInnerTransactionResult {
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
            let field_len = <LazyInnerTransactionResultResult as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyInnerTransactionResultExt as LazyXdr>::xdr_validate(
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
        pos += <LazyInnerTransactionResultResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyInnerTransactionResultExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyInnerTransactionResult {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyInnerTransactionResult {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyInnerTransactionResult {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyInnerTransactionResult {
    /// Access field `fee_charged`.
    #[must_use]
    pub fn fee_charged(&self) -> i64 {
        <i64 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `result`.
    #[must_use]
    pub fn result(&self) -> LazyInnerTransactionResultResult {
        <LazyInnerTransactionResultResult as LazyXdr>::from_xdr_at(&self.0, 8)
    }
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazyInnerTransactionResultExt {
        let buf = self.0.as_slice();
        let mut pos: u32 = 8;
        pos += <LazyInnerTransactionResultResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyInnerTransactionResultExt as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&InnerTransactionResult> for LazyInnerTransactionResult {
    type Error = Error;
    fn try_from(val: &InnerTransactionResult) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyInnerTransactionResult> for InnerTransactionResult {
    type Error = Error;
    fn try_from(lazy: &LazyInnerTransactionResult) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
