#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// CryptoKeyType is an XDR Enum defined as:
///
/// ```text
/// enum CryptoKeyType
/// {
///     KEY_TYPE_ED25519 = 0,
///     KEY_TYPE_PRE_AUTH_TX = 1,
///     KEY_TYPE_HASH_X = 2,
///     KEY_TYPE_ED25519_SIGNED_PAYLOAD = 3,
///     // MUXED enum values for supported type are derived from the enum values
///     // above by ORing them with 0x100
///     KEY_TYPE_MUXED_ED25519 = 0x100
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
pub enum CryptoKeyType {
    #[cfg_attr(feature = "alloc", default)]
    Ed25519 = 0,
    PreAuthTx = 1,
    HashX = 2,
    Ed25519SignedPayload = 3,
    MuxedEd25519 = 256,
}

impl CryptoKeyType {
    const _VARIANTS: &[CryptoKeyType] = &[
        CryptoKeyType::Ed25519,
        CryptoKeyType::PreAuthTx,
        CryptoKeyType::HashX,
        CryptoKeyType::Ed25519SignedPayload,
        CryptoKeyType::MuxedEd25519,
    ];
    pub const VARIANTS: [CryptoKeyType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Ed25519",
        "PreAuthTx",
        "HashX",
        "Ed25519SignedPayload",
        "MuxedEd25519",
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
            Self::Ed25519 => "Ed25519",
            Self::PreAuthTx => "PreAuthTx",
            Self::HashX => "HashX",
            Self::Ed25519SignedPayload => "Ed25519SignedPayload",
            Self::MuxedEd25519 => "MuxedEd25519",
        }
    }

    #[must_use]
    pub const fn variants() -> [CryptoKeyType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for CryptoKeyType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<CryptoKeyType> for CryptoKeyType {
    fn variants() -> slice::Iter<'static, CryptoKeyType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for CryptoKeyType {}

impl fmt::Display for CryptoKeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for CryptoKeyType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => CryptoKeyType::Ed25519,
            1 => CryptoKeyType::PreAuthTx,
            2 => CryptoKeyType::HashX,
            3 => CryptoKeyType::Ed25519SignedPayload,
            256 => CryptoKeyType::MuxedEd25519,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<CryptoKeyType> for i32 {
    #[must_use]
    fn from(e: CryptoKeyType) -> Self {
        e as Self
    }
}

impl ReadXdr for CryptoKeyType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for CryptoKeyType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}

#[cfg(feature = "alloc")]
// Enum CryptoKeyType: scalar lazy type — impl LazyXdr directly on the enum.
impl LazyXdr for CryptoKeyType {
    const FIXED_XDR_SIZE: Option<u32> = Some(4);

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let v = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let _ = CryptoKeyType::try_from(v)?;
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
        CryptoKeyType::try_from(v).unwrap()
    }
}
