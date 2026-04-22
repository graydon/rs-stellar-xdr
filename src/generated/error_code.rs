#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ErrorCode is an XDR Enum defined as:
///
/// ```text
/// enum ErrorCode
/// {
///     ERR_MISC = 0, // Unspecific error
///     ERR_DATA = 1, // Malformed data
///     ERR_CONF = 2, // Misconfiguration error
///     ERR_AUTH = 3, // Authentication failure
///     ERR_LOAD = 4  // System overloaded
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
pub enum ErrorCode {
    #[cfg_attr(feature = "alloc", default)]
    Misc = 0,
    Data = 1,
    Conf = 2,
    Auth = 3,
    Load = 4,
}

impl ErrorCode {
    const _VARIANTS: &[ErrorCode] = &[
        ErrorCode::Misc,
        ErrorCode::Data,
        ErrorCode::Conf,
        ErrorCode::Auth,
        ErrorCode::Load,
    ];
    pub const VARIANTS: [ErrorCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Misc", "Data", "Conf", "Auth", "Load"];
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
            Self::Misc => "Misc",
            Self::Data => "Data",
            Self::Conf => "Conf",
            Self::Auth => "Auth",
            Self::Load => "Load",
        }
    }

    #[must_use]
    pub const fn variants() -> [ErrorCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ErrorCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ErrorCode> for ErrorCode {
    fn variants() -> slice::Iter<'static, ErrorCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ErrorCode {}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ErrorCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ErrorCode::Misc,
            1 => ErrorCode::Data,
            2 => ErrorCode::Conf,
            3 => ErrorCode::Auth,
            4 => ErrorCode::Load,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ErrorCode> for i32 {
    #[must_use]
    fn from(e: ErrorCode) -> Self {
        e as Self
    }
}

impl ReadXdr for ErrorCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ErrorCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}

#[cfg(feature = "alloc")]
// Enum ErrorCode: scalar lazy type — impl LazyXdr directly on the enum.
impl LazyXdr for ErrorCode {
    const FIXED_XDR_SIZE: Option<u32> = Some(4);

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let v = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let _ = ErrorCode::try_from(v)?;
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
        ErrorCode::try_from(v).unwrap()
    }
}
