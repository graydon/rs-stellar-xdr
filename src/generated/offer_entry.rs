#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// OfferEntry is an XDR Struct defined as:
///
/// ```text
/// struct OfferEntry
/// {
///     AccountID sellerID;
///     int64 offerID;
///     Asset selling; // A
///     Asset buying;  // B
///     int64 amount;  // amount of A
///
///     /* price for this offer:
///         price of A in terms of B
///         price=AmountB/AmountA=priceNumerator/priceDenominator
///         price is after fees
///     */
///     Price price;
///     uint32 flags; // see OfferEntryFlags
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
pub struct OfferEntry {
    pub seller_id: AccountId,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub offer_id: i64,
    pub selling: Asset,
    pub buying: Asset,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub amount: i64,
    pub price: Price,
    pub flags: u32,
    pub ext: OfferEntryExt,
}

impl ReadXdr for OfferEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                seller_id: AccountId::read_xdr(r)?,
                offer_id: i64::read_xdr(r)?,
                selling: Asset::read_xdr(r)?,
                buying: Asset::read_xdr(r)?,
                amount: i64::read_xdr(r)?,
                price: Price::read_xdr(r)?,
                flags: u32::read_xdr(r)?,
                ext: OfferEntryExt::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for OfferEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.seller_id.write_xdr(w)?;
            self.offer_id.write_xdr(w)?;
            self.selling.write_xdr(w)?;
            self.buying.write_xdr(w)?;
            self.amount.write_xdr(w)?;
            self.price.write_xdr(w)?;
            self.flags.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`OfferEntry`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyOfferEntry(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyOfferEntry {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyOfferEntry {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.seller_id().cmp(&other.seller_id()))
            .then_with(|| self.offer_id().cmp(&other.offer_id()))
            .then_with(|| self.selling().cmp(&other.selling()))
            .then_with(|| self.buying().cmp(&other.buying()))
            .then_with(|| self.amount().cmp(&other.amount()))
            .then_with(|| self.price().cmp(&other.price()))
            .then_with(|| self.flags().cmp(&other.flags()))
            .then_with(|| self.ext().cmp(&other.ext()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyOfferEntry {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazyAccountId as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
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
        {
            let field_len = <LazyAsset as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(20).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyPrice as LazyXdr>::xdr_validate(&buf[(pos + 8) as usize..], depth)?;
        pos = next_pos;
        {
            let field_len =
                <LazyOfferEntryExt as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 20;
        pos += <LazyOfferEntryExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyOfferEntry {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyOfferEntry {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyOfferEntry {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyOfferEntry {
    /// Access field `seller_id`.
    #[must_use]
    pub fn seller_id(&self) -> LazyAccountId {
        <LazyAccountId as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `offer_id`.
    #[must_use]
    pub fn offer_id(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `selling`.
    #[must_use]
    pub fn selling(&self) -> LazyAsset {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        <LazyAsset as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `buying`.
    #[must_use]
    pub fn buying(&self) -> LazyAsset {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyAsset as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `amount`.
    #[must_use]
    pub fn amount(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `price`.
    #[must_use]
    pub fn price(&self) -> LazyPrice {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        <LazyPrice as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `flags`.
    #[must_use]
    pub fn flags(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 16;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazyOfferEntryExt {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 20;
        <LazyOfferEntryExt as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&OfferEntry> for LazyOfferEntry {
    type Error = Error;
    fn try_from(val: &OfferEntry) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyOfferEntry> for OfferEntry {
    type Error = Error;
    fn try_from(lazy: &LazyOfferEntry) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
