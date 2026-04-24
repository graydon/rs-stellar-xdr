#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// TransactionMetaV2 is an XDR Struct defined as:
///
/// ```text
/// struct TransactionMetaV2
/// {
///     LedgerEntryChanges txChangesBefore; // tx level changes before operations
///                                         // are applied if any
///     OperationMeta operations<>;         // meta for each operation
///     LedgerEntryChanges txChangesAfter;  // tx level changes after operations are
///                                         // applied if any
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
pub struct TransactionMetaV2 {
    pub tx_changes_before: LedgerEntryChanges,
    pub operations: VecM<OperationMeta>,
    pub tx_changes_after: LedgerEntryChanges,
}

impl ReadXdr for TransactionMetaV2 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                tx_changes_before: LedgerEntryChanges::read_xdr(r)?,
                operations: VecM::<OperationMeta>::read_xdr(r)?,
                tx_changes_after: LedgerEntryChanges::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionMetaV2 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx_changes_before.write_xdr(w)?;
            self.operations.write_xdr(w)?;
            self.tx_changes_after.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`TransactionMetaV2`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyTransactionMetaV2(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyTransactionMetaV2 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyTransactionMetaV2 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.tx_changes_before().cmp(&other.tx_changes_before()))
            .then_with(|| self.operations().cmp(&other.operations()))
            .then_with(|| self.tx_changes_after().cmp(&other.tx_changes_after()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyTransactionMetaV2 {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyLedgerEntryChanges as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyVecM<LazyOperationMeta> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
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
        pos += <LazyLedgerEntryChanges as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyOperationMeta> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerEntryChanges as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyTransactionMetaV2 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyTransactionMetaV2 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyTransactionMetaV2 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyTransactionMetaV2 {
    /// Access field `tx_changes_before`.
    #[must_use]
    pub fn tx_changes_before(&self) -> LazyLedgerEntryChanges {
        <LazyLedgerEntryChanges as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `operations`.
    #[must_use]
    pub fn operations(&self) -> LazyVecM<LazyOperationMeta> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyLedgerEntryChanges as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyOperationMeta> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `tx_changes_after`.
    #[must_use]
    pub fn tx_changes_after(&self) -> LazyLedgerEntryChanges {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyLedgerEntryChanges as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyOperationMeta> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyLedgerEntryChanges as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&TransactionMetaV2> for LazyTransactionMetaV2 {
    type Error = Error;
    fn try_from(val: &TransactionMetaV2) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyTransactionMetaV2> for TransactionMetaV2 {
    type Error = Error;
    fn try_from(lazy: &LazyTransactionMetaV2) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
