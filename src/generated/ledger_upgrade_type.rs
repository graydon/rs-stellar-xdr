#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// LedgerUpgradeType is an XDR Enum defined as:
///
/// ```text
/// enum LedgerUpgradeType
/// {
///     LEDGER_UPGRADE_VERSION = 1,
///     LEDGER_UPGRADE_BASE_FEE = 2,
///     LEDGER_UPGRADE_MAX_TX_SET_SIZE = 3,
///     LEDGER_UPGRADE_BASE_RESERVE = 4,
///     LEDGER_UPGRADE_FLAGS = 5,
///     LEDGER_UPGRADE_CONFIG = 6,
///     LEDGER_UPGRADE_MAX_SOROBAN_TX_SET_SIZE = 7
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
pub enum LedgerUpgradeType {
    #[cfg_attr(feature = "alloc", default)]
    Version = 1,
    BaseFee = 2,
    MaxTxSetSize = 3,
    BaseReserve = 4,
    Flags = 5,
    Config = 6,
    MaxSorobanTxSetSize = 7,
}

impl LedgerUpgradeType {
    const _VARIANTS: &[LedgerUpgradeType] = &[
        LedgerUpgradeType::Version,
        LedgerUpgradeType::BaseFee,
        LedgerUpgradeType::MaxTxSetSize,
        LedgerUpgradeType::BaseReserve,
        LedgerUpgradeType::Flags,
        LedgerUpgradeType::Config,
        LedgerUpgradeType::MaxSorobanTxSetSize,
    ];
    pub const VARIANTS: [LedgerUpgradeType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Version",
        "BaseFee",
        "MaxTxSetSize",
        "BaseReserve",
        "Flags",
        "Config",
        "MaxSorobanTxSetSize",
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
            Self::Version => "Version",
            Self::BaseFee => "BaseFee",
            Self::MaxTxSetSize => "MaxTxSetSize",
            Self::BaseReserve => "BaseReserve",
            Self::Flags => "Flags",
            Self::Config => "Config",
            Self::MaxSorobanTxSetSize => "MaxSorobanTxSetSize",
        }
    }

    #[must_use]
    pub const fn variants() -> [LedgerUpgradeType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for LedgerUpgradeType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<LedgerUpgradeType> for LedgerUpgradeType {
    fn variants() -> slice::Iter<'static, LedgerUpgradeType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for LedgerUpgradeType {}

impl fmt::Display for LedgerUpgradeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for LedgerUpgradeType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            1 => LedgerUpgradeType::Version,
            2 => LedgerUpgradeType::BaseFee,
            3 => LedgerUpgradeType::MaxTxSetSize,
            4 => LedgerUpgradeType::BaseReserve,
            5 => LedgerUpgradeType::Flags,
            6 => LedgerUpgradeType::Config,
            7 => LedgerUpgradeType::MaxSorobanTxSetSize,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<LedgerUpgradeType> for i32 {
    #[must_use]
    fn from(e: LedgerUpgradeType) -> Self {
        e as Self
    }
}

impl ReadXdr for LedgerUpgradeType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for LedgerUpgradeType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}

#[cfg(feature = "alloc")]
// Enum LedgerUpgradeType: scalar lazy type — impl LazyXdr directly on the enum.
impl LazyXdr for LedgerUpgradeType {
    const FIXED_XDR_SIZE: Option<u32> = Some(4);

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let v = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let _ = LedgerUpgradeType::try_from(v)?;
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
        LedgerUpgradeType::try_from(v).unwrap()
    }
}
