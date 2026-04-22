#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// LedgerEntryType is an XDR Enum defined as:
///
/// ```text
/// enum LedgerEntryType
/// {
///     ACCOUNT = 0,
///     TRUSTLINE = 1,
///     OFFER = 2,
///     DATA = 3,
///     CLAIMABLE_BALANCE = 4,
///     LIQUIDITY_POOL = 5,
///     CONTRACT_DATA = 6,
///     CONTRACT_CODE = 7,
///     CONFIG_SETTING = 8,
///     TTL = 9
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
pub enum LedgerEntryType {
    #[cfg_attr(feature = "alloc", default)]
    Account = 0,
    Trustline = 1,
    Offer = 2,
    Data = 3,
    ClaimableBalance = 4,
    LiquidityPool = 5,
    ContractData = 6,
    ContractCode = 7,
    ConfigSetting = 8,
    Ttl = 9,
}

impl LedgerEntryType {
    const _VARIANTS: &[LedgerEntryType] = &[
        LedgerEntryType::Account,
        LedgerEntryType::Trustline,
        LedgerEntryType::Offer,
        LedgerEntryType::Data,
        LedgerEntryType::ClaimableBalance,
        LedgerEntryType::LiquidityPool,
        LedgerEntryType::ContractData,
        LedgerEntryType::ContractCode,
        LedgerEntryType::ConfigSetting,
        LedgerEntryType::Ttl,
    ];
    pub const VARIANTS: [LedgerEntryType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Account",
        "Trustline",
        "Offer",
        "Data",
        "ClaimableBalance",
        "LiquidityPool",
        "ContractData",
        "ContractCode",
        "ConfigSetting",
        "Ttl",
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
            Self::Account => "Account",
            Self::Trustline => "Trustline",
            Self::Offer => "Offer",
            Self::Data => "Data",
            Self::ClaimableBalance => "ClaimableBalance",
            Self::LiquidityPool => "LiquidityPool",
            Self::ContractData => "ContractData",
            Self::ContractCode => "ContractCode",
            Self::ConfigSetting => "ConfigSetting",
            Self::Ttl => "Ttl",
        }
    }

    #[must_use]
    pub const fn variants() -> [LedgerEntryType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for LedgerEntryType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<LedgerEntryType> for LedgerEntryType {
    fn variants() -> slice::Iter<'static, LedgerEntryType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for LedgerEntryType {}

impl fmt::Display for LedgerEntryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for LedgerEntryType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => LedgerEntryType::Account,
            1 => LedgerEntryType::Trustline,
            2 => LedgerEntryType::Offer,
            3 => LedgerEntryType::Data,
            4 => LedgerEntryType::ClaimableBalance,
            5 => LedgerEntryType::LiquidityPool,
            6 => LedgerEntryType::ContractData,
            7 => LedgerEntryType::ContractCode,
            8 => LedgerEntryType::ConfigSetting,
            9 => LedgerEntryType::Ttl,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<LedgerEntryType> for i32 {
    #[must_use]
    fn from(e: LedgerEntryType) -> Self {
        e as Self
    }
}

impl ReadXdr for LedgerEntryType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for LedgerEntryType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}

#[cfg(feature = "alloc")]
// Enum LedgerEntryType: scalar lazy type — impl LazyXdr directly on the enum.
impl LazyXdr for LedgerEntryType {
    const FIXED_XDR_SIZE: Option<u32> = Some(4);

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let v = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let _ = LedgerEntryType::try_from(v)?;
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
        LedgerEntryType::try_from(v).unwrap()
    }
}
