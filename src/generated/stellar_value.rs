#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// StellarValue is an XDR Struct defined as:
///
/// ```text
/// struct StellarValue
/// {
///     Hash txSetHash;      // transaction set to apply to previous ledger
///     TimePoint closeTime; // network close time
///
///     // upgrades to apply to the previous ledger (usually empty)
///     // this is a vector of encoded 'LedgerUpgrade' so that nodes can drop
///     // unknown steps during consensus if needed.
///     // see notes below on 'LedgerUpgrade' for more detail
///     // max size is dictated by number of upgrade types (+ room for future)
///     UpgradeType upgrades<6>;
///
///     // reserved for future use
///     union switch (StellarValueType v)
///     {
///     case STELLAR_VALUE_BASIC:
///         void;
///     case STELLAR_VALUE_SIGNED:
///         LedgerCloseValueSignature lcValueSignature;
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
pub struct StellarValue {
    pub tx_set_hash: Hash,
    pub close_time: TimePoint,
    pub upgrades: VecM<UpgradeType, 6>,
    pub ext: StellarValueExt,
}

impl ReadXdr for StellarValue {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                tx_set_hash: Hash::read_xdr(r)?,
                close_time: TimePoint::read_xdr(r)?,
                upgrades: VecM::<UpgradeType, 6>::read_xdr(r)?,
                ext: StellarValueExt::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for StellarValue {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx_set_hash.write_xdr(w)?;
            self.close_time.write_xdr(w)?;
            self.upgrades.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`StellarValue`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyStellarValue(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyStellarValue {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyStellarValue {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.tx_set_hash().cmp(&other.tx_set_hash()))
            .then_with(|| self.close_time().cmp(&other.close_time()))
            .then_with(|| self.upgrades().cmp(&other.upgrades()))
            .then_with(|| self.ext().cmp(&other.ext()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyStellarValue {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(40).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyHash as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        {
            let field_len = <LazyVecM<LazyUpgradeType, 6> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyStellarValueExt as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += 40;
        pos += <LazyVecM<LazyUpgradeType, 6> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStellarValueExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyStellarValue {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyStellarValue {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyStellarValue {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyStellarValue {
    /// Access field `tx_set_hash`.
    #[must_use]
    pub fn tx_set_hash(&self) -> LazyHash {
        <LazyHash as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `close_time`.
    #[must_use]
    pub fn close_time(&self) -> LazyTimePoint {
        <LazyTimePoint as LazyXdr>::from_xdr_at(&self.0, 32)
    }
    /// Access field `upgrades`.
    #[must_use]
    pub fn upgrades(&self) -> LazyVecM<LazyUpgradeType, 6> {
        <LazyVecM<LazyUpgradeType, 6> as LazyXdr>::from_xdr_at(&self.0, 40)
    }
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazyStellarValueExt {
        let buf = self.0.as_slice();
        let mut pos: u32 = 40;
        pos += <LazyVecM<LazyUpgradeType, 6> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyStellarValueExt as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&StellarValue> for LazyStellarValue {
    type Error = Error;
    fn try_from(val: &StellarValue) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyStellarValue> for StellarValue {
    type Error = Error;
    fn try_from(lazy: &LazyStellarValue) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
