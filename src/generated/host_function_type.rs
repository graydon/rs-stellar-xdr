#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// HostFunctionType is an XDR Enum defined as:
///
/// ```text
/// enum HostFunctionType
/// {
///     HOST_FUNCTION_TYPE_INVOKE_CONTRACT = 0,
///     HOST_FUNCTION_TYPE_CREATE_CONTRACT = 1,
///     HOST_FUNCTION_TYPE_UPLOAD_CONTRACT_WASM = 2,
///     HOST_FUNCTION_TYPE_CREATE_CONTRACT_V2 = 3
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
pub enum HostFunctionType {
    #[cfg_attr(feature = "alloc", default)]
    InvokeContract = 0,
    CreateContract = 1,
    UploadContractWasm = 2,
    CreateContractV2 = 3,
}

impl HostFunctionType {
    const _VARIANTS: &[HostFunctionType] = &[
        HostFunctionType::InvokeContract,
        HostFunctionType::CreateContract,
        HostFunctionType::UploadContractWasm,
        HostFunctionType::CreateContractV2,
    ];
    pub const VARIANTS: [HostFunctionType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "InvokeContract",
        "CreateContract",
        "UploadContractWasm",
        "CreateContractV2",
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
            Self::InvokeContract => "InvokeContract",
            Self::CreateContract => "CreateContract",
            Self::UploadContractWasm => "UploadContractWasm",
            Self::CreateContractV2 => "CreateContractV2",
        }
    }

    #[must_use]
    pub const fn variants() -> [HostFunctionType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for HostFunctionType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<HostFunctionType> for HostFunctionType {
    fn variants() -> slice::Iter<'static, HostFunctionType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for HostFunctionType {}

impl fmt::Display for HostFunctionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for HostFunctionType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => HostFunctionType::InvokeContract,
            1 => HostFunctionType::CreateContract,
            2 => HostFunctionType::UploadContractWasm,
            3 => HostFunctionType::CreateContractV2,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<HostFunctionType> for i32 {
    #[must_use]
    fn from(e: HostFunctionType) -> Self {
        e as Self
    }
}

impl ReadXdr for HostFunctionType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for HostFunctionType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}

#[cfg(feature = "alloc")]
// Enum HostFunctionType: scalar lazy type — impl LazyXdr directly on the enum.
impl LazyXdr for HostFunctionType {
    const FIXED_XDR_SIZE: Option<u32> = Some(4);

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let v = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let _ = HostFunctionType::try_from(v)?;
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
        HostFunctionType::try_from(v).unwrap()
    }
}
