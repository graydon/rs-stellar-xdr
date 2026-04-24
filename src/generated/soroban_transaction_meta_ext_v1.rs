#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SorobanTransactionMetaExtV1 is an XDR Struct defined as:
///
/// ```text
/// struct SorobanTransactionMetaExtV1
/// {
///     ExtensionPoint ext;
///
///     // The following are the components of the overall Soroban resource fee
///     // charged for the transaction.
///     // The following relation holds:
///     // `resourceFeeCharged = totalNonRefundableResourceFeeCharged + totalRefundableResourceFeeCharged`
///     // where `resourceFeeCharged` is the overall fee charged for the
///     // transaction. Also, `resourceFeeCharged` <= `sorobanData.resourceFee`
///     // i.e.we never charge more than the declared resource fee.
///     // The inclusion fee for charged the Soroban transaction can be found using
///     // the following equation:
///     // `result.feeCharged = resourceFeeCharged + inclusionFeeCharged`.
///
///     // Total amount (in stroops) that has been charged for non-refundable
///     // Soroban resources.
///     // Non-refundable resources are charged based on the usage declared in
///     // the transaction envelope (such as `instructions`, `readBytes` etc.) and
///     // is charged regardless of the success of the transaction.
///     int64 totalNonRefundableResourceFeeCharged;
///     // Total amount (in stroops) that has been charged for refundable
///     // Soroban resource fees.
///     // Currently this comprises the rent fee (`rentFeeCharged`) and the
///     // fee for the events and return value.
///     // Refundable resources are charged based on the actual resources usage.
///     // Since currently refundable resources are only used for the successful
///     // transactions, this will be `0` for failed transactions.
///     int64 totalRefundableResourceFeeCharged;
///     // Amount (in stroops) that has been charged for rent.
///     // This is a part of `totalNonRefundableResourceFeeCharged`.
///     int64 rentFeeCharged;
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
pub struct SorobanTransactionMetaExtV1 {
    pub ext: ExtensionPoint,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub total_non_refundable_resource_fee_charged: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub total_refundable_resource_fee_charged: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub rent_fee_charged: i64,
}

impl ReadXdr for SorobanTransactionMetaExtV1 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ExtensionPoint::read_xdr(r)?,
                total_non_refundable_resource_fee_charged: i64::read_xdr(r)?,
                total_refundable_resource_fee_charged: i64::read_xdr(r)?,
                rent_fee_charged: i64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SorobanTransactionMetaExtV1 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.total_non_refundable_resource_fee_charged
                .write_xdr(w)?;
            self.total_refundable_resource_fee_charged.write_xdr(w)?;
            self.rent_fee_charged.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`SorobanTransactionMetaExtV1`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySorobanTransactionMetaExtV1(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazySorobanTransactionMetaExtV1 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazySorobanTransactionMetaExtV1 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.ext().cmp(&other.ext()))
            .then_with(|| {
                self.total_non_refundable_resource_fee_charged()
                    .cmp(&other.total_non_refundable_resource_fee_charged())
            })
            .then_with(|| {
                self.total_refundable_resource_fee_charged()
                    .cmp(&other.total_refundable_resource_fee_charged())
            })
            .then_with(|| self.rent_fee_charged().cmp(&other.rent_fee_charged()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazySorobanTransactionMetaExtV1 {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyExtensionPoint as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(24).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 24;
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
impl From<LazyHandle> for LazySorobanTransactionMetaExtV1 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazySorobanTransactionMetaExtV1 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazySorobanTransactionMetaExtV1 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazySorobanTransactionMetaExtV1 {
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazyExtensionPoint {
        <LazyExtensionPoint as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `total_non_refundable_resource_fee_charged`.
    #[must_use]
    pub fn total_non_refundable_resource_fee_charged(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `total_refundable_resource_fee_charged`.
    #[must_use]
    pub fn total_refundable_resource_fee_charged(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `rent_fee_charged`.
    #[must_use]
    pub fn rent_fee_charged(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 16;
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&SorobanTransactionMetaExtV1> for LazySorobanTransactionMetaExtV1 {
    type Error = Error;
    fn try_from(val: &SorobanTransactionMetaExtV1) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazySorobanTransactionMetaExtV1> for SorobanTransactionMetaExtV1 {
    type Error = Error;
    fn try_from(lazy: &LazySorobanTransactionMetaExtV1) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
