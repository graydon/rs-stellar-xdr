#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// TransactionMetaV4 is an XDR Struct defined as:
///
/// ```text
/// struct TransactionMetaV4
/// {
///     ExtensionPoint ext;
///
///     LedgerEntryChanges txChangesBefore;  // tx level changes before operations
///                                          // are applied if any
///     OperationMetaV2 operations<>;        // meta for each operation
///     LedgerEntryChanges txChangesAfter;   // tx level changes after operations are
///                                          // applied if any
///     SorobanTransactionMetaV2* sorobanMeta; // Soroban-specific meta (only for
///                                            // Soroban transactions).
///
///     TransactionEvent events<>; // Used for transaction-level events (like fee payment)
///     DiagnosticEvent diagnosticEvents<>; // Used for all diagnostic information
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
pub struct TransactionMetaV4 {
    pub ext: ExtensionPoint,
    pub tx_changes_before: LedgerEntryChanges,
    pub operations: VecM<OperationMetaV2>,
    pub tx_changes_after: LedgerEntryChanges,
    pub soroban_meta: Option<SorobanTransactionMetaV2>,
    pub events: VecM<TransactionEvent>,
    pub diagnostic_events: VecM<DiagnosticEvent>,
}

impl ReadXdr for TransactionMetaV4 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ExtensionPoint::read_xdr(r)?,
                tx_changes_before: LedgerEntryChanges::read_xdr(r)?,
                operations: VecM::<OperationMetaV2>::read_xdr(r)?,
                tx_changes_after: LedgerEntryChanges::read_xdr(r)?,
                soroban_meta: Option::<SorobanTransactionMetaV2>::read_xdr(r)?,
                events: VecM::<TransactionEvent>::read_xdr(r)?,
                diagnostic_events: VecM::<DiagnosticEvent>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionMetaV4 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.tx_changes_before.write_xdr(w)?;
            self.operations.write_xdr(w)?;
            self.tx_changes_after.write_xdr(w)?;
            self.soroban_meta.write_xdr(w)?;
            self.events.write_xdr(w)?;
            self.diagnostic_events.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`TransactionMetaV4`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyTransactionMetaV4(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyTransactionMetaV4 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyTransactionMetaV4 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.ext().cmp(&other.ext()))
            .then_with(|| self.tx_changes_before().cmp(&other.tx_changes_before()))
            .then_with(|| self.operations().cmp(&other.operations()))
            .then_with(|| self.tx_changes_after().cmp(&other.tx_changes_after()))
            .then_with(|| self.soroban_meta().cmp(&other.soroban_meta()))
            .then_with(|| self.events().cmp(&other.events()))
            .then_with(|| self.diagnostic_events().cmp(&other.diagnostic_events()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyTransactionMetaV4 {
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
                <LazyLedgerEntryChanges as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyVecM<LazyOperationMetaV2> as LazyXdr>::xdr_validate(
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
        {
            let field_len = <LazyOption<LazySorobanTransactionMetaV2> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyVecM<LazyTransactionEvent> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyVecM<LazyDiagnosticEvent> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerEntryChanges as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyOperationMetaV2> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerEntryChanges as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazySorobanTransactionMetaV2> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyTransactionEvent> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyDiagnosticEvent> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyTransactionMetaV4 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyTransactionMetaV4 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyTransactionMetaV4 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyTransactionMetaV4 {
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazyExtensionPoint {
        <LazyExtensionPoint as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `tx_changes_before`.
    #[must_use]
    pub fn tx_changes_before(&self) -> LazyLedgerEntryChanges {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyLedgerEntryChanges as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `operations`.
    #[must_use]
    pub fn operations(&self) -> LazyVecM<LazyOperationMetaV2> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerEntryChanges as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyOperationMetaV2> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `tx_changes_after`.
    #[must_use]
    pub fn tx_changes_after(&self) -> LazyLedgerEntryChanges {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerEntryChanges as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyOperationMetaV2> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyLedgerEntryChanges as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `soroban_meta`.
    #[must_use]
    pub fn soroban_meta(&self) -> LazyOption<LazySorobanTransactionMetaV2> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerEntryChanges as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyOperationMetaV2> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerEntryChanges as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyOption<LazySorobanTransactionMetaV2> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `events`.
    #[must_use]
    pub fn events(&self) -> LazyVecM<LazyTransactionEvent> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerEntryChanges as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyOperationMetaV2> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerEntryChanges as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazySorobanTransactionMetaV2> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyTransactionEvent> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `diagnostic_events`.
    #[must_use]
    pub fn diagnostic_events(&self) -> LazyVecM<LazyDiagnosticEvent> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerEntryChanges as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyOperationMetaV2> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerEntryChanges as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazySorobanTransactionMetaV2> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyTransactionEvent> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyDiagnosticEvent> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&TransactionMetaV4> for LazyTransactionMetaV4 {
    type Error = Error;
    fn try_from(val: &TransactionMetaV4) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyTransactionMetaV4> for TransactionMetaV4 {
    type Error = Error;
    fn try_from(lazy: &LazyTransactionMetaV4) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
