#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ClaimPredicate is an XDR Union defined as:
///
/// ```text
/// union ClaimPredicate switch (ClaimPredicateType type)
/// {
/// case CLAIM_PREDICATE_UNCONDITIONAL:
///     void;
/// case CLAIM_PREDICATE_AND:
///     ClaimPredicate andPredicates<2>;
/// case CLAIM_PREDICATE_OR:
///     ClaimPredicate orPredicates<2>;
/// case CLAIM_PREDICATE_NOT:
///     ClaimPredicate* notPredicate;
/// case CLAIM_PREDICATE_BEFORE_ABSOLUTE_TIME:
///     int64 absBefore; // Predicate will be true if closeTime < absBefore
/// case CLAIM_PREDICATE_BEFORE_RELATIVE_TIME:
///     int64 relBefore; // Seconds since closeTime of the ledger in which the
///                      // ClaimableBalanceEntry was created
/// };
/// ```
///
// union with discriminant ClaimPredicateType
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
pub enum ClaimPredicate {
    Unconditional,
    And(VecM<ClaimPredicate, 2>),
    Or(VecM<ClaimPredicate, 2>),
    Not(Option<Box<ClaimPredicate>>),
    BeforeAbsoluteTime(
        #[cfg_attr(
            all(feature = "serde", feature = "alloc"),
            serde_as(as = "NumberOrString")
        )]
        i64,
    ),
    BeforeRelativeTime(
        #[cfg_attr(
            all(feature = "serde", feature = "alloc"),
            serde_as(as = "NumberOrString")
        )]
        i64,
    ),
}

#[cfg(feature = "alloc")]
impl Default for ClaimPredicate {
    fn default() -> Self {
        Self::Unconditional
    }
}

impl ClaimPredicate {
    const _VARIANTS: &[ClaimPredicateType] = &[
        ClaimPredicateType::Unconditional,
        ClaimPredicateType::And,
        ClaimPredicateType::Or,
        ClaimPredicateType::Not,
        ClaimPredicateType::BeforeAbsoluteTime,
        ClaimPredicateType::BeforeRelativeTime,
    ];
    pub const VARIANTS: [ClaimPredicateType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Unconditional",
        "And",
        "Or",
        "Not",
        "BeforeAbsoluteTime",
        "BeforeRelativeTime",
    ];
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
            Self::Unconditional => "Unconditional",
            Self::And(_) => "And",
            Self::Or(_) => "Or",
            Self::Not(_) => "Not",
            Self::BeforeAbsoluteTime(_) => "BeforeAbsoluteTime",
            Self::BeforeRelativeTime(_) => "BeforeRelativeTime",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ClaimPredicateType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Unconditional => ClaimPredicateType::Unconditional,
            Self::And(_) => ClaimPredicateType::And,
            Self::Or(_) => ClaimPredicateType::Or,
            Self::Not(_) => ClaimPredicateType::Not,
            Self::BeforeAbsoluteTime(_) => ClaimPredicateType::BeforeAbsoluteTime,
            Self::BeforeRelativeTime(_) => ClaimPredicateType::BeforeRelativeTime,
        }
    }

    #[must_use]
    pub const fn variants() -> [ClaimPredicateType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ClaimPredicate {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ClaimPredicateType> for ClaimPredicate {
    #[must_use]
    fn discriminant(&self) -> ClaimPredicateType {
        Self::discriminant(self)
    }
}

impl Variants<ClaimPredicateType> for ClaimPredicate {
    fn variants() -> slice::Iter<'static, ClaimPredicateType> {
        Self::VARIANTS.iter()
    }
}

impl Union<ClaimPredicateType> for ClaimPredicate {}

impl ReadXdr for ClaimPredicate {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ClaimPredicateType = <ClaimPredicateType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ClaimPredicateType::Unconditional => Self::Unconditional,
                ClaimPredicateType::And => Self::And(VecM::<ClaimPredicate, 2>::read_xdr(r)?),
                ClaimPredicateType::Or => Self::Or(VecM::<ClaimPredicate, 2>::read_xdr(r)?),
                ClaimPredicateType::Not => Self::Not(Option::<Box<ClaimPredicate>>::read_xdr(r)?),
                ClaimPredicateType::BeforeAbsoluteTime => {
                    Self::BeforeAbsoluteTime(i64::read_xdr(r)?)
                }
                ClaimPredicateType::BeforeRelativeTime => {
                    Self::BeforeRelativeTime(i64::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ClaimPredicate {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Unconditional => ().write_xdr(w)?,
                Self::And(v) => v.write_xdr(w)?,
                Self::Or(v) => v.write_xdr(w)?,
                Self::Not(v) => v.write_xdr(w)?,
                Self::BeforeAbsoluteTime(v) => v.write_xdr(w)?,
                Self::BeforeRelativeTime(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ClaimPredicate`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyClaimPredicate(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyClaimPredicate {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyClaimPredicate {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            1 => self.as_and().cmp(&other.as_and()),
            2 => self.as_or().cmp(&other.as_or()),
            3 => self.as_not().cmp(&other.as_not()),
            4 => self
                .as_before_absolute_time()
                .cmp(&other.as_before_absolute_time()),
            5 => self
                .as_before_relative_time()
                .cmp(&other.as_before_relative_time()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyClaimPredicate {
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
                // void — no additional data
            }
            1 => {
                let field_len = <LazyVecM<LazyClaimPredicate, 2> as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            2 => {
                let field_len = <LazyVecM<LazyClaimPredicate, 2> as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            3 => {
                let field_len = <LazyOption<LazyClaimPredicate> as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            4 => {
                let field_len = <i64 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            5 => {
                let field_len = <i64 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
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
                // void
            }
            1 => {
                pos += <LazyVecM<LazyClaimPredicate, 2> as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            2 => {
                pos += <LazyVecM<LazyClaimPredicate, 2> as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            3 => {
                pos += <LazyOption<LazyClaimPredicate> as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            4 => {
                pos += <i64 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            5 => {
                pos += <i64 as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyClaimPredicate {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyClaimPredicate {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyClaimPredicate {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyClaimPredicate {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> ClaimPredicateType {
        // Validated — unwrap is safe.
        ClaimPredicateType::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `And`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_and(&self) -> Option<LazyVecM<LazyClaimPredicate, 2>> {
        if self.discriminant_i32() == 1 {
            Some(<LazyVecM<LazyClaimPredicate, 2> as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `Or`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_or(&self) -> Option<LazyVecM<LazyClaimPredicate, 2>> {
        if self.discriminant_i32() == 2 {
            Some(<LazyVecM<LazyClaimPredicate, 2> as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `Not`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_not(&self) -> Option<LazyOption<LazyClaimPredicate>> {
        if self.discriminant_i32() == 3 {
            Some(<LazyOption<LazyClaimPredicate> as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `BeforeAbsoluteTime`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_before_absolute_time(&self) -> Option<i64> {
        if self.discriminant_i32() == 4 {
            Some(<i64 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `BeforeRelativeTime`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_before_relative_time(&self) -> Option<i64> {
        if self.discriminant_i32() == 5 {
            Some(<i64 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ClaimPredicate> for LazyClaimPredicate {
    type Error = Error;
    fn try_from(val: &ClaimPredicate) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyClaimPredicate> for ClaimPredicate {
    type Error = Error;
    fn try_from(lazy: &LazyClaimPredicate) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
