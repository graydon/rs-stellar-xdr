#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ClaimPredicateType is an XDR Enum defined as:
///
/// ```text
/// enum ClaimPredicateType
/// {
///     CLAIM_PREDICATE_UNCONDITIONAL = 0,
///     CLAIM_PREDICATE_AND = 1,
///     CLAIM_PREDICATE_OR = 2,
///     CLAIM_PREDICATE_NOT = 3,
///     CLAIM_PREDICATE_BEFORE_ABSOLUTE_TIME = 4,
///     CLAIM_PREDICATE_BEFORE_RELATIVE_TIME = 5
/// };
/// ```
///
// enum
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[repr(i32)]
pub enum ClaimPredicateType {
    #[cfg_attr(feature = "alloc", default)]
    Unconditional = 0,
    And = 1,
    Or = 2,
    Not = 3,
    BeforeAbsoluteTime = 4,
    BeforeRelativeTime = 5,
}

impl ClaimPredicateType {
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
            Self::And => "And",
            Self::Or => "Or",
            Self::Not => "Not",
            Self::BeforeAbsoluteTime => "BeforeAbsoluteTime",
            Self::BeforeRelativeTime => "BeforeRelativeTime",
        }
    }

    #[must_use]
    pub const fn variants() -> [ClaimPredicateType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ClaimPredicateType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ClaimPredicateType> for ClaimPredicateType {
    fn variants() -> slice::Iter<'static, ClaimPredicateType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ClaimPredicateType {}

impl fmt::Display for ClaimPredicateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ClaimPredicateType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ClaimPredicateType::Unconditional,
            1 => ClaimPredicateType::And,
            2 => ClaimPredicateType::Or,
            3 => ClaimPredicateType::Not,
            4 => ClaimPredicateType::BeforeAbsoluteTime,
            5 => ClaimPredicateType::BeforeRelativeTime,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ClaimPredicateType> for i32 {
    #[must_use]
    fn from(e: ClaimPredicateType) -> Self {
        e as Self
    }
}

impl ReadXdr for ClaimPredicateType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ClaimPredicateType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}

#[cfg(feature = "alloc")]
// Enum ClaimPredicateType: scalar lazy type — impl LazyXdr directly on the enum.
impl LazyXdr for ClaimPredicateType {
    const FIXED_XDR_SIZE: Option<u32> = Some(4);

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let v = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let _ = ClaimPredicateType::try_from(v)?;
        Ok(4)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        4
    }

    #[inline]
    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let b = &parent.as_slice()[offset as usize..];
        let v = i32::from_be_bytes([b[0], b[1], b[2], b[3]]);
        // SAFETY: data was validated; unwrap is infallible.
        ClaimPredicateType::try_from(v).unwrap()
    }
}
