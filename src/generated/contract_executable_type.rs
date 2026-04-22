#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ContractExecutableType is an XDR Enum defined as:
///
/// ```text
/// enum ContractExecutableType
/// {
///     CONTRACT_EXECUTABLE_WASM = 0,
///     CONTRACT_EXECUTABLE_STELLAR_ASSET = 1
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
pub enum ContractExecutableType {
    #[cfg_attr(feature = "alloc", default)]
    Wasm = 0,
    StellarAsset = 1,
}

impl ContractExecutableType {
    const _VARIANTS: &[ContractExecutableType] = &[
        ContractExecutableType::Wasm,
        ContractExecutableType::StellarAsset,
    ];
    pub const VARIANTS: [ContractExecutableType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Wasm", "StellarAsset"];
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
            Self::Wasm => "Wasm",
            Self::StellarAsset => "StellarAsset",
        }
    }

    #[must_use]
    pub const fn variants() -> [ContractExecutableType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ContractExecutableType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ContractExecutableType> for ContractExecutableType {
    fn variants() -> slice::Iter<'static, ContractExecutableType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ContractExecutableType {}

impl fmt::Display for ContractExecutableType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ContractExecutableType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ContractExecutableType::Wasm,
            1 => ContractExecutableType::StellarAsset,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ContractExecutableType> for i32 {
    #[must_use]
    fn from(e: ContractExecutableType) -> Self {
        e as Self
    }
}

impl ReadXdr for ContractExecutableType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ContractExecutableType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}

#[cfg(feature = "alloc")]
// Enum ContractExecutableType: scalar lazy type — impl LazyXdr directly on the enum.
impl LazyXdr for ContractExecutableType {
    const FIXED_XDR_SIZE: Option<u32> = Some(4);

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let v = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let _ = ContractExecutableType::try_from(v)?;
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
        ContractExecutableType::try_from(v).unwrap()
    }
}
