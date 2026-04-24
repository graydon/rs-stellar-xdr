#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// InvokeHostFunctionResultCode is an XDR Enum defined as:
///
/// ```text
/// enum InvokeHostFunctionResultCode
/// {
///     // codes considered as "success" for the operation
///     INVOKE_HOST_FUNCTION_SUCCESS = 0,
///
///     // codes considered as "failure" for the operation
///     INVOKE_HOST_FUNCTION_MALFORMED = -1,
///     INVOKE_HOST_FUNCTION_TRAPPED = -2,
///     INVOKE_HOST_FUNCTION_RESOURCE_LIMIT_EXCEEDED = -3,
///     INVOKE_HOST_FUNCTION_ENTRY_ARCHIVED = -4,
///     INVOKE_HOST_FUNCTION_INSUFFICIENT_REFUNDABLE_FEE = -5
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
pub enum InvokeHostFunctionResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    Malformed = -1,
    Trapped = -2,
    ResourceLimitExceeded = -3,
    EntryArchived = -4,
    InsufficientRefundableFee = -5,
}

impl InvokeHostFunctionResultCode {
    const _VARIANTS: &[InvokeHostFunctionResultCode] = &[
        InvokeHostFunctionResultCode::Success,
        InvokeHostFunctionResultCode::Malformed,
        InvokeHostFunctionResultCode::Trapped,
        InvokeHostFunctionResultCode::ResourceLimitExceeded,
        InvokeHostFunctionResultCode::EntryArchived,
        InvokeHostFunctionResultCode::InsufficientRefundableFee,
    ];
    pub const VARIANTS: [InvokeHostFunctionResultCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Success",
        "Malformed",
        "Trapped",
        "ResourceLimitExceeded",
        "EntryArchived",
        "InsufficientRefundableFee",
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
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::Trapped => "Trapped",
            Self::ResourceLimitExceeded => "ResourceLimitExceeded",
            Self::EntryArchived => "EntryArchived",
            Self::InsufficientRefundableFee => "InsufficientRefundableFee",
        }
    }

    #[must_use]
    pub const fn variants() -> [InvokeHostFunctionResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for InvokeHostFunctionResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<InvokeHostFunctionResultCode> for InvokeHostFunctionResultCode {
    fn variants() -> slice::Iter<'static, InvokeHostFunctionResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for InvokeHostFunctionResultCode {}

impl fmt::Display for InvokeHostFunctionResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for InvokeHostFunctionResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => InvokeHostFunctionResultCode::Success,
            -1 => InvokeHostFunctionResultCode::Malformed,
            -2 => InvokeHostFunctionResultCode::Trapped,
            -3 => InvokeHostFunctionResultCode::ResourceLimitExceeded,
            -4 => InvokeHostFunctionResultCode::EntryArchived,
            -5 => InvokeHostFunctionResultCode::InsufficientRefundableFee,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<InvokeHostFunctionResultCode> for i32 {
    #[must_use]
    fn from(e: InvokeHostFunctionResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for InvokeHostFunctionResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for InvokeHostFunctionResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}

#[cfg(feature = "alloc")]
// Enum InvokeHostFunctionResultCode: scalar lazy type — impl LazyXdr directly on the enum.
impl LazyXdr for InvokeHostFunctionResultCode {
    const FIXED_XDR_SIZE: Option<u32> = Some(4);

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let bytes: [u8; 4] = buf[..4].try_into().unwrap();
        let v = i32::from_be_bytes(bytes);
        let _ = InvokeHostFunctionResultCode::try_from(v)?;
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
        InvokeHostFunctionResultCode::try_from(v).unwrap()
    }
}
