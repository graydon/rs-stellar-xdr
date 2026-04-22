#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// LedgerEntryData is an XDR NestedUnion defined as:
///
/// ```text
/// union switch (LedgerEntryType type)
///     {
///     case ACCOUNT:
///         AccountEntry account;
///     case TRUSTLINE:
///         TrustLineEntry trustLine;
///     case OFFER:
///         OfferEntry offer;
///     case DATA:
///         DataEntry data;
///     case CLAIMABLE_BALANCE:
///         ClaimableBalanceEntry claimableBalance;
///     case LIQUIDITY_POOL:
///         LiquidityPoolEntry liquidityPool;
///     case CONTRACT_DATA:
///         ContractDataEntry contractData;
///     case CONTRACT_CODE:
///         ContractCodeEntry contractCode;
///     case CONFIG_SETTING:
///         ConfigSettingEntry configSetting;
///     case TTL:
///         TTLEntry ttl;
///     }
/// ```
///
// union with discriminant LedgerEntryType
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[allow(clippy::large_enum_variant)]
pub enum LedgerEntryData {
    Account(AccountEntry),
    Trustline(TrustLineEntry),
    Offer(OfferEntry),
    Data(DataEntry),
    ClaimableBalance(ClaimableBalanceEntry),
    LiquidityPool(LiquidityPoolEntry),
    ContractData(ContractDataEntry),
    ContractCode(ContractCodeEntry),
    ConfigSetting(ConfigSettingEntry),
    Ttl(TtlEntry),
}

#[cfg(feature = "alloc")]
impl Default for LedgerEntryData {
    fn default() -> Self {
        Self::Account(AccountEntry::default())
    }
}

impl LedgerEntryData {
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
            Self::Account(_) => "Account",
            Self::Trustline(_) => "Trustline",
            Self::Offer(_) => "Offer",
            Self::Data(_) => "Data",
            Self::ClaimableBalance(_) => "ClaimableBalance",
            Self::LiquidityPool(_) => "LiquidityPool",
            Self::ContractData(_) => "ContractData",
            Self::ContractCode(_) => "ContractCode",
            Self::ConfigSetting(_) => "ConfigSetting",
            Self::Ttl(_) => "Ttl",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> LedgerEntryType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Account(_) => LedgerEntryType::Account,
            Self::Trustline(_) => LedgerEntryType::Trustline,
            Self::Offer(_) => LedgerEntryType::Offer,
            Self::Data(_) => LedgerEntryType::Data,
            Self::ClaimableBalance(_) => LedgerEntryType::ClaimableBalance,
            Self::LiquidityPool(_) => LedgerEntryType::LiquidityPool,
            Self::ContractData(_) => LedgerEntryType::ContractData,
            Self::ContractCode(_) => LedgerEntryType::ContractCode,
            Self::ConfigSetting(_) => LedgerEntryType::ConfigSetting,
            Self::Ttl(_) => LedgerEntryType::Ttl,
        }
    }

    #[must_use]
    pub const fn variants() -> [LedgerEntryType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for LedgerEntryData {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<LedgerEntryType> for LedgerEntryData {
    #[must_use]
    fn discriminant(&self) -> LedgerEntryType {
        Self::discriminant(self)
    }
}

impl Variants<LedgerEntryType> for LedgerEntryData {
    fn variants() -> slice::Iter<'static, LedgerEntryType> {
        Self::VARIANTS.iter()
    }
}

impl Union<LedgerEntryType> for LedgerEntryData {}

impl ReadXdr for LedgerEntryData {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: LedgerEntryType = <LedgerEntryType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                LedgerEntryType::Account => Self::Account(AccountEntry::read_xdr(r)?),
                LedgerEntryType::Trustline => Self::Trustline(TrustLineEntry::read_xdr(r)?),
                LedgerEntryType::Offer => Self::Offer(OfferEntry::read_xdr(r)?),
                LedgerEntryType::Data => Self::Data(DataEntry::read_xdr(r)?),
                LedgerEntryType::ClaimableBalance => {
                    Self::ClaimableBalance(ClaimableBalanceEntry::read_xdr(r)?)
                }
                LedgerEntryType::LiquidityPool => {
                    Self::LiquidityPool(LiquidityPoolEntry::read_xdr(r)?)
                }
                LedgerEntryType::ContractData => {
                    Self::ContractData(ContractDataEntry::read_xdr(r)?)
                }
                LedgerEntryType::ContractCode => {
                    Self::ContractCode(ContractCodeEntry::read_xdr(r)?)
                }
                LedgerEntryType::ConfigSetting => {
                    Self::ConfigSetting(ConfigSettingEntry::read_xdr(r)?)
                }
                LedgerEntryType::Ttl => Self::Ttl(TtlEntry::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for LedgerEntryData {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Account(v) => v.write_xdr(w)?,
                Self::Trustline(v) => v.write_xdr(w)?,
                Self::Offer(v) => v.write_xdr(w)?,
                Self::Data(v) => v.write_xdr(w)?,
                Self::ClaimableBalance(v) => v.write_xdr(w)?,
                Self::LiquidityPool(v) => v.write_xdr(w)?,
                Self::ContractData(v) => v.write_xdr(w)?,
                Self::ContractCode(v) => v.write_xdr(w)?,
                Self::ConfigSetting(v) => v.write_xdr(w)?,
                Self::Ttl(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`LedgerEntryData`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyLedgerEntryData(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyLedgerEntryData {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyLedgerEntryData {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_account().cmp(&other.as_account()),
            1 => self.as_trustline().cmp(&other.as_trustline()),
            2 => self.as_offer().cmp(&other.as_offer()),
            3 => self.as_data().cmp(&other.as_data()),
            4 => self
                .as_claimable_balance()
                .cmp(&other.as_claimable_balance()),
            5 => self.as_liquidity_pool().cmp(&other.as_liquidity_pool()),
            6 => self.as_contract_data().cmp(&other.as_contract_data()),
            7 => self.as_contract_code().cmp(&other.as_contract_code()),
            8 => self.as_config_setting().cmp(&other.as_config_setting()),
            9 => self.as_ttl().cmp(&other.as_ttl()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyLedgerEntryData {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let disc = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        #[allow(unused_mut)]
        let mut pos: u32 = 4;
        #[allow(clippy::match_same_arms)]
        match disc {
            0 => {
                let field_len =
                    <LazyAccountEntry as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            1 => {
                let field_len =
                    <LazyTrustLineEntry as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            2 => {
                let field_len =
                    <LazyOfferEntry as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            3 => {
                let field_len =
                    <LazyDataEntry as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            4 => {
                let field_len = <LazyClaimableBalanceEntry as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            5 => {
                let field_len =
                    <LazyLiquidityPoolEntry as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            6 => {
                let field_len =
                    <LazyContractDataEntry as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            7 => {
                let field_len =
                    <LazyContractCodeEntry as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            8 => {
                let field_len =
                    <LazyConfigSettingEntry as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            9 => {
                let field_len =
                    <LazyTtlEntry as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            _ => return Err(Error::Invalid),
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let disc = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        #[allow(unused_mut)]
        let mut pos: u32 = 4;
        #[allow(clippy::match_same_arms)]
        match disc {
            0 => {
                pos += <LazyAccountEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            1 => {
                pos += <LazyTrustLineEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            2 => {
                pos += <LazyOfferEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            3 => {
                pos += <LazyDataEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            4 => {
                pos += <LazyClaimableBalanceEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            5 => {
                pos += <LazyLiquidityPoolEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            6 => {
                pos += <LazyContractDataEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            7 => {
                pos += <LazyContractCodeEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            8 => {
                pos += <LazyConfigSettingEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            9 => {
                pos += <LazyTtlEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            _ => {}
        }
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyLedgerEntryData {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyLedgerEntryData {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyLedgerEntryData {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyLedgerEntryData {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> LedgerEntryType {
        // Validated — unwrap is safe.
        LedgerEntryType::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `Account`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_account(&self) -> Option<LazyAccountEntry> {
        if self.discriminant_i32() == 0 {
            Some(<LazyAccountEntry as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Trustline`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_trustline(&self) -> Option<LazyTrustLineEntry> {
        if self.discriminant_i32() == 1 {
            Some(<LazyTrustLineEntry as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Offer`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_offer(&self) -> Option<LazyOfferEntry> {
        if self.discriminant_i32() == 2 {
            Some(<LazyOfferEntry as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Data`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_data(&self) -> Option<LazyDataEntry> {
        if self.discriminant_i32() == 3 {
            Some(<LazyDataEntry as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ClaimableBalance`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_claimable_balance(&self) -> Option<LazyClaimableBalanceEntry> {
        if self.discriminant_i32() == 4 {
            Some(<LazyClaimableBalanceEntry as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `LiquidityPool`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_liquidity_pool(&self) -> Option<LazyLiquidityPoolEntry> {
        if self.discriminant_i32() == 5 {
            Some(<LazyLiquidityPoolEntry as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ContractData`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_contract_data(&self) -> Option<LazyContractDataEntry> {
        if self.discriminant_i32() == 6 {
            Some(<LazyContractDataEntry as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ContractCode`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_contract_code(&self) -> Option<LazyContractCodeEntry> {
        if self.discriminant_i32() == 7 {
            Some(<LazyContractCodeEntry as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ConfigSetting`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_config_setting(&self) -> Option<LazyConfigSettingEntry> {
        if self.discriminant_i32() == 8 {
            Some(<LazyConfigSettingEntry as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Ttl`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_ttl(&self) -> Option<LazyTtlEntry> {
        if self.discriminant_i32() == 9 {
            Some(<LazyTtlEntry as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LedgerEntryData> for LazyLedgerEntryData {
    type Error = Error;
    fn try_from(val: &LedgerEntryData) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyLedgerEntryData> for LedgerEntryData {
    type Error = Error;
    fn try_from(lazy: &LazyLedgerEntryData) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
