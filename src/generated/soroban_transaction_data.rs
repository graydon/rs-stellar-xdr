#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SorobanTransactionData is an XDR Struct defined as:
///
/// ```text
/// struct SorobanTransactionData
/// {
///     union switch (int v)
///     {
///     case 0:
///         void;
///     case 1:
///         SorobanResourcesExtV0 resourceExt;
///     } ext;
///     SorobanResources resources;
///     // Amount of the transaction `fee` allocated to the Soroban resource fees.
///     // The fraction of `resourceFee` corresponding to `resources` specified
///     // above is *not* refundable (i.e. fees for instructions, ledger I/O), as
///     // well as fees for the transaction size.
///     // The remaining part of the fee is refundable and the charged value is
///     // based on the actual consumption of refundable resources (events, ledger
///     // rent bumps).
///     // The `inclusionFee` used for prioritization of the transaction is defined
///     // as `tx.fee - resourceFee`.
///     int64 resourceFee;
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
pub struct SorobanTransactionData {
    pub ext: SorobanTransactionDataExt,
    pub resources: SorobanResources,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub resource_fee: i64,
}

impl ReadXdr for SorobanTransactionData {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: SorobanTransactionDataExt::read_xdr(r)?,
                resources: SorobanResources::read_xdr(r)?,
                resource_fee: i64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SorobanTransactionData {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.resources.write_xdr(w)?;
            self.resource_fee.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`SorobanTransactionData`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySorobanTransactionData(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazySorobanTransactionData {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazySorobanTransactionData {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.ext().cmp(&other.ext()))
            .then_with(|| self.resources().cmp(&other.resources()))
            .then_with(|| self.resource_fee().cmp(&other.resource_fee()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazySorobanTransactionData {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazySorobanTransactionDataExt as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazySorobanResources as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(8).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazySorobanTransactionDataExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazySorobanResources as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazySorobanTransactionData {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazySorobanTransactionData {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazySorobanTransactionData {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazySorobanTransactionData {
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazySorobanTransactionDataExt {
        <LazySorobanTransactionDataExt as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `resources`.
    #[must_use]
    pub fn resources(&self) -> LazySorobanResources {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazySorobanTransactionDataExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazySorobanResources as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `resource_fee`.
    #[must_use]
    pub fn resource_fee(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazySorobanTransactionDataExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazySorobanResources as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&SorobanTransactionData> for LazySorobanTransactionData {
    type Error = Error;
    fn try_from(val: &SorobanTransactionData) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazySorobanTransactionData> for SorobanTransactionData {
    type Error = Error;
    fn try_from(lazy: &LazySorobanTransactionData) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
