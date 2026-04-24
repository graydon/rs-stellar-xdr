#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ScSpecEventParamLocationV0 is an XDR Enum defined as:
///
/// ```text
/// enum SCSpecEventParamLocationV0
/// {
///     SC_SPEC_EVENT_PARAM_LOCATION_DATA = 0,
///     SC_SPEC_EVENT_PARAM_LOCATION_TOPIC_LIST = 1
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
pub enum ScSpecEventParamLocationV0 {
    #[cfg_attr(feature = "alloc", default)]
    Data = 0,
    TopicList = 1,
}

impl ScSpecEventParamLocationV0 {
    const _VARIANTS: &[ScSpecEventParamLocationV0] = &[
        ScSpecEventParamLocationV0::Data,
        ScSpecEventParamLocationV0::TopicList,
    ];
    pub const VARIANTS: [ScSpecEventParamLocationV0; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Data", "TopicList"];
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
            Self::Data => "Data",
            Self::TopicList => "TopicList",
        }
    }

    #[must_use]
    pub const fn variants() -> [ScSpecEventParamLocationV0; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScSpecEventParamLocationV0 {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ScSpecEventParamLocationV0> for ScSpecEventParamLocationV0 {
    fn variants() -> slice::Iter<'static, ScSpecEventParamLocationV0> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ScSpecEventParamLocationV0 {}

impl fmt::Display for ScSpecEventParamLocationV0 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ScSpecEventParamLocationV0 {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ScSpecEventParamLocationV0::Data,
            1 => ScSpecEventParamLocationV0::TopicList,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ScSpecEventParamLocationV0> for i32 {
    #[must_use]
    fn from(e: ScSpecEventParamLocationV0) -> Self {
        e as Self
    }
}

impl ReadXdr for ScSpecEventParamLocationV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ScSpecEventParamLocationV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}

#[cfg(feature = "alloc")]
// Enum ScSpecEventParamLocationV0: scalar lazy type — impl LazyXdr directly on the enum.
impl LazyXdr for ScSpecEventParamLocationV0 {
    const FIXED_XDR_SIZE: Option<u32> = Some(4);

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let bytes: [u8; 4] = buf[..4].try_into().unwrap();
        let v = i32::from_be_bytes(bytes);
        let _ = ScSpecEventParamLocationV0::try_from(v)?;
        Ok(4)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        4
    }

    #[inline]
    fn from_xdr_consume(_parent: &LazyHandle, buf: &mut &[u8]) -> Self {
        let bytes: [u8; 4] = buf[..4].try_into().unwrap();
        *buf = &buf[4..];
        let v = i32::from_be_bytes(bytes);
        // SAFETY: data was validated; unwrap is infallible.
        ScSpecEventParamLocationV0::try_from(v).unwrap()
    }
}
