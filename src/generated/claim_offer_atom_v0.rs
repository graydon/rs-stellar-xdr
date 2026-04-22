#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ClaimOfferAtomV0 is an XDR Struct defined as:
///
/// ```text
/// struct ClaimOfferAtomV0
/// {
///     // emitted to identify the offer
///     uint256 sellerEd25519; // Account that owns the offer
///     int64 offerID;
///
///     // amount and asset taken from the owner
///     Asset assetSold;
///     int64 amountSold;
///
///     // amount and asset sent to the owner
///     Asset assetBought;
///     int64 amountBought;
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
pub struct ClaimOfferAtomV0 {
    pub seller_ed25519: Uint256,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub offer_id: i64,
    pub asset_sold: Asset,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub amount_sold: i64,
    pub asset_bought: Asset,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub amount_bought: i64,
}

impl ReadXdr for ClaimOfferAtomV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                seller_ed25519: Uint256::read_xdr(r)?,
                offer_id: i64::read_xdr(r)?,
                asset_sold: Asset::read_xdr(r)?,
                amount_sold: i64::read_xdr(r)?,
                asset_bought: Asset::read_xdr(r)?,
                amount_bought: i64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ClaimOfferAtomV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.seller_ed25519.write_xdr(w)?;
            self.offer_id.write_xdr(w)?;
            self.asset_sold.write_xdr(w)?;
            self.amount_sold.write_xdr(w)?;
            self.asset_bought.write_xdr(w)?;
            self.amount_bought.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ClaimOfferAtomV0`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyClaimOfferAtomV0(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyClaimOfferAtomV0 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyClaimOfferAtomV0 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.seller_ed25519().cmp(&other.seller_ed25519()))
            .then_with(|| self.offer_id().cmp(&other.offer_id()))
            .then_with(|| self.asset_sold().cmp(&other.asset_sold()))
            .then_with(|| self.amount_sold().cmp(&other.amount_sold()))
            .then_with(|| self.asset_bought().cmp(&other.asset_bought()))
            .then_with(|| self.amount_bought().cmp(&other.amount_bought()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyClaimOfferAtomV0 {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(40).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyUint256 as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        {
            let field_len = <LazyAsset as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(8).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        {
            let field_len = <LazyAsset as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
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
        pos += 40;
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyClaimOfferAtomV0 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyClaimOfferAtomV0 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyClaimOfferAtomV0 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyClaimOfferAtomV0 {
    /// Access field `seller_ed25519`.
    #[must_use]
    pub fn seller_ed25519(&self) -> LazyUint256 {
        <LazyUint256 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `offer_id`.
    #[must_use]
    pub fn offer_id(&self) -> i64 {
        <i64 as LazyXdr>::from_xdr_at(&self.0, 32)
    }
    /// Access field `asset_sold`.
    #[must_use]
    pub fn asset_sold(&self) -> LazyAsset {
        <LazyAsset as LazyXdr>::from_xdr_at(&self.0, 40)
    }
    /// Access field `amount_sold`.
    #[must_use]
    pub fn amount_sold(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 40;
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `asset_bought`.
    #[must_use]
    pub fn asset_bought(&self) -> LazyAsset {
        let buf = self.0.as_slice();
        let mut pos: u32 = 40;
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        <LazyAsset as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `amount_bought`.
    #[must_use]
    pub fn amount_bought(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 40;
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ClaimOfferAtomV0> for LazyClaimOfferAtomV0 {
    type Error = Error;
    fn try_from(val: &ClaimOfferAtomV0) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyClaimOfferAtomV0> for ClaimOfferAtomV0 {
    type Error = Error;
    fn try_from(lazy: &LazyClaimOfferAtomV0) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
