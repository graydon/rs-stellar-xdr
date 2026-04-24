#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ManageOfferSuccessResult is an XDR Struct defined as:
///
/// ```text
/// struct ManageOfferSuccessResult
/// {
///     // offers that got claimed while creating this offer
///     ClaimAtom offersClaimed<>;
///
///     union switch (ManageOfferEffect effect)
///     {
///     case MANAGE_OFFER_CREATED:
///     case MANAGE_OFFER_UPDATED:
///         OfferEntry offer;
///     case MANAGE_OFFER_DELETED:
///         void;
///     }
///     offer;
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
pub struct ManageOfferSuccessResult {
    pub offers_claimed: VecM<ClaimAtom>,
    pub offer: ManageOfferSuccessResultOffer,
}

impl ReadXdr for ManageOfferSuccessResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                offers_claimed: VecM::<ClaimAtom>::read_xdr(r)?,
                offer: ManageOfferSuccessResultOffer::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ManageOfferSuccessResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.offers_claimed.write_xdr(w)?;
            self.offer.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ManageOfferSuccessResult`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyManageOfferSuccessResult(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyManageOfferSuccessResult {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyManageOfferSuccessResult {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.offers_claimed().cmp(&other.offers_claimed()))
            .then_with(|| self.offer().cmp(&other.offer()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyManageOfferSuccessResult {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyVecM<LazyClaimAtom> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyManageOfferSuccessResultOffer as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyVecM<LazyClaimAtom> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyManageOfferSuccessResultOffer as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyManageOfferSuccessResult {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyManageOfferSuccessResult {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyManageOfferSuccessResult {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyManageOfferSuccessResult {
    /// Access field `offers_claimed`.
    #[must_use]
    pub fn offers_claimed(&self) -> LazyVecM<LazyClaimAtom> {
        <LazyVecM<LazyClaimAtom> as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `offer`.
    #[must_use]
    pub fn offer(&self) -> LazyManageOfferSuccessResultOffer {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyVecM<LazyClaimAtom> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyManageOfferSuccessResultOffer as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ManageOfferSuccessResult> for LazyManageOfferSuccessResult {
    type Error = Error;
    fn try_from(val: &ManageOfferSuccessResult) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyManageOfferSuccessResult> for ManageOfferSuccessResult {
    type Error = Error;
    fn try_from(lazy: &LazyManageOfferSuccessResult) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
