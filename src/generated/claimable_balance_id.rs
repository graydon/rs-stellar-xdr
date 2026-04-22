#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ClaimableBalanceId is an XDR Union defined as:
///
/// ```text
/// union ClaimableBalanceID switch (ClaimableBalanceIDType type)
/// {
/// case CLAIMABLE_BALANCE_ID_TYPE_V0:
///     Hash v0;
/// };
/// ```
///
// union with discriminant ClaimableBalanceIdType
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
#[allow(clippy::large_enum_variant)]
pub enum ClaimableBalanceId {
    ClaimableBalanceIdTypeV0(Hash),
}

#[cfg(feature = "alloc")]
impl Default for ClaimableBalanceId {
    fn default() -> Self {
        Self::ClaimableBalanceIdTypeV0(Hash::default())
    }
}

impl ClaimableBalanceId {
    const _VARIANTS: &[ClaimableBalanceIdType] =
        &[ClaimableBalanceIdType::ClaimableBalanceIdTypeV0];
    pub const VARIANTS: [ClaimableBalanceIdType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["ClaimableBalanceIdTypeV0"];
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
            Self::ClaimableBalanceIdTypeV0(_) => "ClaimableBalanceIdTypeV0",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ClaimableBalanceIdType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ClaimableBalanceIdTypeV0(_) => ClaimableBalanceIdType::ClaimableBalanceIdTypeV0,
        }
    }

    #[must_use]
    pub const fn variants() -> [ClaimableBalanceIdType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ClaimableBalanceId {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ClaimableBalanceIdType> for ClaimableBalanceId {
    #[must_use]
    fn discriminant(&self) -> ClaimableBalanceIdType {
        Self::discriminant(self)
    }
}

impl Variants<ClaimableBalanceIdType> for ClaimableBalanceId {
    fn variants() -> slice::Iter<'static, ClaimableBalanceIdType> {
        Self::VARIANTS.iter()
    }
}

impl Union<ClaimableBalanceIdType> for ClaimableBalanceId {}

impl ReadXdr for ClaimableBalanceId {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ClaimableBalanceIdType = <ClaimableBalanceIdType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ClaimableBalanceIdType::ClaimableBalanceIdTypeV0 => {
                    Self::ClaimableBalanceIdTypeV0(Hash::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ClaimableBalanceId {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::ClaimableBalanceIdTypeV0(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ClaimableBalanceId`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyClaimableBalanceId(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyClaimableBalanceId {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyClaimableBalanceId {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self
                .as_claimable_balance_id_type_v0()
                .cmp(&other.as_claimable_balance_id_type_v0()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyClaimableBalanceId {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let disc = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        #[allow(unused_mut)]
        let mut pos: u32 = 4;
        #[allow(clippy::match_same_arms)]
        match disc {
            0 => {
                let field_len = <LazyHash as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            _ => return Err(Error::Invalid),
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let disc = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        #[allow(unused_mut)]
        let mut pos: u32 = 4;
        #[allow(clippy::match_same_arms)]
        match disc {
            0 => {
                pos += <LazyHash as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            _ => {}
        }
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyClaimableBalanceId {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyClaimableBalanceId {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyClaimableBalanceId {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyClaimableBalanceId {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> ClaimableBalanceIdType {
        // Validated — unwrap is safe.
        ClaimableBalanceIdType::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `ClaimableBalanceIdTypeV0`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_claimable_balance_id_type_v0(&self) -> Option<LazyHash> {
        if self.discriminant_i32() == 0 {
            Some(<LazyHash as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ClaimableBalanceId> for LazyClaimableBalanceId {
    type Error = Error;
    fn try_from(val: &ClaimableBalanceId) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyClaimableBalanceId> for ClaimableBalanceId {
    type Error = Error;
    fn try_from(lazy: &LazyClaimableBalanceId) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
