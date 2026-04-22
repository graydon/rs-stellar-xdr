#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// BucketListType is an XDR Enum defined as:
///
/// ```text
/// enum BucketListType
/// {
///     LIVE = 0,
///     HOT_ARCHIVE = 1
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
pub enum BucketListType {
    #[cfg_attr(feature = "alloc", default)]
    Live = 0,
    HotArchive = 1,
}

impl BucketListType {
    const _VARIANTS: &[BucketListType] = &[BucketListType::Live, BucketListType::HotArchive];
    pub const VARIANTS: [BucketListType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Live", "HotArchive"];
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
            Self::Live => "Live",
            Self::HotArchive => "HotArchive",
        }
    }

    #[must_use]
    pub const fn variants() -> [BucketListType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for BucketListType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<BucketListType> for BucketListType {
    fn variants() -> slice::Iter<'static, BucketListType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for BucketListType {}

impl fmt::Display for BucketListType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for BucketListType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => BucketListType::Live,
            1 => BucketListType::HotArchive,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<BucketListType> for i32 {
    #[must_use]
    fn from(e: BucketListType) -> Self {
        e as Self
    }
}

impl ReadXdr for BucketListType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for BucketListType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}

#[cfg(feature = "alloc")]
// Enum BucketListType: scalar lazy type — impl LazyXdr directly on the enum.
impl LazyXdr for BucketListType {
    const FIXED_XDR_SIZE: Option<u32> = Some(4);

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let v = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let _ = BucketListType::try_from(v)?;
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
        BucketListType::try_from(v).unwrap()
    }
}
