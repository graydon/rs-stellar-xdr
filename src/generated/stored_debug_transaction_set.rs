#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// StoredDebugTransactionSet is an XDR Struct defined as:
///
/// ```text
/// struct StoredDebugTransactionSet
/// {
/// 	StoredTransactionSet txSet;
/// 	uint32 ledgerSeq;
/// 	StellarValue scpValue;
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
pub struct StoredDebugTransactionSet {
    pub tx_set: StoredTransactionSet,
    pub ledger_seq: u32,
    pub scp_value: StellarValue,
}

impl ReadXdr for StoredDebugTransactionSet {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                tx_set: StoredTransactionSet::read_xdr(r)?,
                ledger_seq: u32::read_xdr(r)?,
                scp_value: StellarValue::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for StoredDebugTransactionSet {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx_set.write_xdr(w)?;
            self.ledger_seq.write_xdr(w)?;
            self.scp_value.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`StoredDebugTransactionSet`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyStoredDebugTransactionSet(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyStoredDebugTransactionSet {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyStoredDebugTransactionSet {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.tx_set().cmp(&other.tx_set()))
            .then_with(|| self.ledger_seq().cmp(&other.ledger_seq()))
            .then_with(|| self.scp_value().cmp(&other.scp_value()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyStoredDebugTransactionSet {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyStoredTransactionSet as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(4).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        {
            let field_len =
                <LazyStellarValue as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyStoredTransactionSet as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        pos += <LazyStellarValue as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyStoredDebugTransactionSet {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyStoredDebugTransactionSet {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyStoredDebugTransactionSet {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyStoredDebugTransactionSet {
    /// Access field `tx_set`.
    #[must_use]
    pub fn tx_set(&self) -> LazyStoredTransactionSet {
        <LazyStoredTransactionSet as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `ledger_seq`.
    #[must_use]
    pub fn ledger_seq(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyStoredTransactionSet as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `scp_value`.
    #[must_use]
    pub fn scp_value(&self) -> LazyStellarValue {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyStoredTransactionSet as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        <LazyStellarValue as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&StoredDebugTransactionSet> for LazyStoredDebugTransactionSet {
    type Error = Error;
    fn try_from(val: &StoredDebugTransactionSet) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyStoredDebugTransactionSet> for StoredDebugTransactionSet {
    type Error = Error;
    fn try_from(lazy: &LazyStoredDebugTransactionSet) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
