#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// TransactionResultMetaV1 is an XDR Struct defined as:
///
/// ```text
/// struct TransactionResultMetaV1
/// {
///     ExtensionPoint ext;
///
///     TransactionResultPair result;
///     LedgerEntryChanges feeProcessing;
///     TransactionMeta txApplyProcessing;
///
///     LedgerEntryChanges postTxApplyFeeProcessing;
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
pub struct TransactionResultMetaV1 {
    pub ext: ExtensionPoint,
    pub result: TransactionResultPair,
    pub fee_processing: LedgerEntryChanges,
    pub tx_apply_processing: TransactionMeta,
    pub post_tx_apply_fee_processing: LedgerEntryChanges,
}

impl ReadXdr for TransactionResultMetaV1 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ExtensionPoint::read_xdr(r)?,
                result: TransactionResultPair::read_xdr(r)?,
                fee_processing: LedgerEntryChanges::read_xdr(r)?,
                tx_apply_processing: TransactionMeta::read_xdr(r)?,
                post_tx_apply_fee_processing: LedgerEntryChanges::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionResultMetaV1 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.result.write_xdr(w)?;
            self.fee_processing.write_xdr(w)?;
            self.tx_apply_processing.write_xdr(w)?;
            self.post_tx_apply_fee_processing.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`TransactionResultMetaV1`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyTransactionResultMetaV1(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyTransactionResultMetaV1 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyTransactionResultMetaV1 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.ext().cmp(&other.ext()))
            .then_with(|| self.result().cmp(&other.result()))
            .then_with(|| self.fee_processing().cmp(&other.fee_processing()))
            .then_with(|| self.tx_apply_processing().cmp(&other.tx_apply_processing()))
            .then_with(|| {
                self.post_tx_apply_fee_processing()
                    .cmp(&other.post_tx_apply_fee_processing())
            })
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyTransactionResultMetaV1 {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyExtensionPoint as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyTransactionResultPair as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyLedgerEntryChanges as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyTransactionMeta as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyLedgerEntryChanges as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyTransactionResultPair as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerEntryChanges as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyTransactionMeta as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerEntryChanges as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyTransactionResultMetaV1 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyTransactionResultMetaV1 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyTransactionResultMetaV1 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyTransactionResultMetaV1 {
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazyExtensionPoint {
        <LazyExtensionPoint as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `result`.
    #[must_use]
    pub fn result(&self) -> LazyTransactionResultPair {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyTransactionResultPair as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `fee_processing`.
    #[must_use]
    pub fn fee_processing(&self) -> LazyLedgerEntryChanges {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyTransactionResultPair as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyLedgerEntryChanges as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `tx_apply_processing`.
    #[must_use]
    pub fn tx_apply_processing(&self) -> LazyTransactionMeta {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyTransactionResultPair as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerEntryChanges as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyTransactionMeta as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `post_tx_apply_fee_processing`.
    #[must_use]
    pub fn post_tx_apply_fee_processing(&self) -> LazyLedgerEntryChanges {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyTransactionResultPair as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerEntryChanges as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyTransactionMeta as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyLedgerEntryChanges as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&TransactionResultMetaV1> for LazyTransactionResultMetaV1 {
    type Error = Error;
    fn try_from(val: &TransactionResultMetaV1) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyTransactionResultMetaV1> for TransactionResultMetaV1 {
    type Error = Error;
    fn try_from(lazy: &LazyTransactionResultMetaV1) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
