#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ScSpecUdtUnionCaseV0Kind is an XDR Enum defined as:
///
/// ```text
/// enum SCSpecUDTUnionCaseV0Kind
/// {
///     SC_SPEC_UDT_UNION_CASE_VOID_V0 = 0,
///     SC_SPEC_UDT_UNION_CASE_TUPLE_V0 = 1
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
pub enum ScSpecUdtUnionCaseV0Kind {
    #[cfg_attr(feature = "alloc", default)]
    VoidV0 = 0,
    TupleV0 = 1,
}

impl ScSpecUdtUnionCaseV0Kind {
    const _VARIANTS: &[ScSpecUdtUnionCaseV0Kind] = &[
        ScSpecUdtUnionCaseV0Kind::VoidV0,
        ScSpecUdtUnionCaseV0Kind::TupleV0,
    ];
    pub const VARIANTS: [ScSpecUdtUnionCaseV0Kind; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["VoidV0", "TupleV0"];
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
            Self::VoidV0 => "VoidV0",
            Self::TupleV0 => "TupleV0",
        }
    }

    #[must_use]
    pub const fn variants() -> [ScSpecUdtUnionCaseV0Kind; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScSpecUdtUnionCaseV0Kind {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ScSpecUdtUnionCaseV0Kind> for ScSpecUdtUnionCaseV0Kind {
    fn variants() -> slice::Iter<'static, ScSpecUdtUnionCaseV0Kind> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ScSpecUdtUnionCaseV0Kind {}

impl fmt::Display for ScSpecUdtUnionCaseV0Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ScSpecUdtUnionCaseV0Kind {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ScSpecUdtUnionCaseV0Kind::VoidV0,
            1 => ScSpecUdtUnionCaseV0Kind::TupleV0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ScSpecUdtUnionCaseV0Kind> for i32 {
    #[must_use]
    fn from(e: ScSpecUdtUnionCaseV0Kind) -> Self {
        e as Self
    }
}

impl ReadXdr for ScSpecUdtUnionCaseV0Kind {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ScSpecUdtUnionCaseV0Kind {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}

#[cfg(feature = "alloc")]
// Enum ScSpecUdtUnionCaseV0Kind: scalar lazy type — impl LazyXdr directly on the enum.
impl LazyXdr for ScSpecUdtUnionCaseV0Kind {
    const FIXED_XDR_SIZE: Option<u32> = Some(4);

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let bytes: [u8; 4] = buf[..4].try_into().unwrap();
        let v = i32::from_be_bytes(bytes);
        let _ = ScSpecUdtUnionCaseV0Kind::try_from(v)?;
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
        ScSpecUdtUnionCaseV0Kind::try_from(v).unwrap()
    }
}
