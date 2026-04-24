#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ManageOfferSuccessResultOffer is an XDR NestedUnion defined as:
///
/// ```text
/// union switch (ManageOfferEffect effect)
///     {
///     case MANAGE_OFFER_CREATED:
///     case MANAGE_OFFER_UPDATED:
///         OfferEntry offer;
///     case MANAGE_OFFER_DELETED:
///         void;
///     }
/// ```
///
// union with discriminant ManageOfferEffect
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[allow(clippy::large_enum_variant)]
pub enum ManageOfferSuccessResultOffer {
    Created(OfferEntry),
    Updated(OfferEntry),
    Deleted,
}

#[cfg(feature = "alloc")]
impl Default for ManageOfferSuccessResultOffer {
    fn default() -> Self {
        Self::Created(OfferEntry::default())
    }
}

impl ManageOfferSuccessResultOffer {
    const _VARIANTS: &[ManageOfferEffect] = &[
        ManageOfferEffect::Created,
        ManageOfferEffect::Updated,
        ManageOfferEffect::Deleted,
    ];
    pub const VARIANTS: [ManageOfferEffect; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Created", "Updated", "Deleted"];
    pub const VARIANTS_STR: [&'static str; Self::_VARIANTS_STR.len()] = {
        let mut arr = [Self::_VARIANTS_STR[0]; Self::_VARIANTS_STR.len()];
        let mut i = 1;
        while i < Self::_VARIANTS_STR.len() {
            arr[i] = Self::_VARIANTS_STR[i];
            i += 1;
        }
        arr
    };

    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Created(_) => "Created",
            Self::Updated(_) => "Updated",
            Self::Deleted => "Deleted",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ManageOfferEffect {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Created(_) => ManageOfferEffect::Created,
            Self::Updated(_) => ManageOfferEffect::Updated,
            Self::Deleted => ManageOfferEffect::Deleted,
        }
    }

    #[must_use]
    pub const fn variants() -> [ManageOfferEffect; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ManageOfferSuccessResultOffer {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ManageOfferEffect> for ManageOfferSuccessResultOffer {
    #[must_use]
    fn discriminant(&self) -> ManageOfferEffect {
        Self::discriminant(self)
    }
}

impl Variants<ManageOfferEffect> for ManageOfferSuccessResultOffer {
    fn variants() -> slice::Iter<'static, ManageOfferEffect> {
        Self::VARIANTS.iter()
    }
}

impl Union<ManageOfferEffect> for ManageOfferSuccessResultOffer {}

impl ReadXdr for ManageOfferSuccessResultOffer {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ManageOfferEffect = <ManageOfferEffect as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ManageOfferEffect::Created => Self::Created(OfferEntry::read_xdr(r)?),
                ManageOfferEffect::Updated => Self::Updated(OfferEntry::read_xdr(r)?),
                ManageOfferEffect::Deleted => Self::Deleted,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ManageOfferSuccessResultOffer {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Created(v) => v.write_xdr(w)?,
                Self::Updated(v) => v.write_xdr(w)?,
                Self::Deleted => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ManageOfferSuccessResultOffer`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyManageOfferSuccessResultOffer(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyManageOfferSuccessResultOffer {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyManageOfferSuccessResultOffer {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_created().cmp(&other.as_created()),
            1 => self.as_updated().cmp(&other.as_updated()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyManageOfferSuccessResultOffer {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let bytes: [u8; 4] = buf[..4].try_into().unwrap();
        let disc = i32::from_be_bytes(bytes);
        #[allow(unused_mut)]
        let mut pos: u32 = 4;
        #[allow(clippy::match_same_arms)]
        match disc {
            0 => {
                let field_len =
                    <LazyOfferEntry as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            1 => {
                let field_len =
                    <LazyOfferEntry as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            2 => {
                // void — no additional data
            }
            _ => return Err(Error::Invalid),
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let bytes: [u8; 4] = buf[..4].try_into().unwrap();
        let disc = i32::from_be_bytes(bytes);
        #[allow(unused_mut)]
        let mut pos: u32 = 4;
        #[allow(clippy::match_same_arms)]
        match disc {
            0 => {
                pos += <LazyOfferEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            1 => {
                pos += <LazyOfferEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            2 => {
                // void
            }
            _ => {}
        }
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
impl From<LazyHandle> for LazyManageOfferSuccessResultOffer {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyManageOfferSuccessResultOffer {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyManageOfferSuccessResultOffer {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyManageOfferSuccessResultOffer {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> ManageOfferEffect {
        // Validated — unwrap is safe.
        ManageOfferEffect::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `Created`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_created(&self) -> Option<LazyOfferEntry> {
        if self.discriminant_i32() == 0 {
            Some(<LazyOfferEntry as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Updated`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_updated(&self) -> Option<LazyOfferEntry> {
        if self.discriminant_i32() == 1 {
            Some(<LazyOfferEntry as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ManageOfferSuccessResultOffer> for LazyManageOfferSuccessResultOffer {
    type Error = Error;
    fn try_from(val: &ManageOfferSuccessResultOffer) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyManageOfferSuccessResultOffer> for ManageOfferSuccessResultOffer {
    type Error = Error;
    fn try_from(lazy: &LazyManageOfferSuccessResultOffer) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
