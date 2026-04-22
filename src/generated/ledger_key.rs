#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// LedgerKey is an XDR Union defined as:
///
/// ```text
/// union LedgerKey switch (LedgerEntryType type)
/// {
/// case ACCOUNT:
///     struct
///     {
///         AccountID accountID;
///     } account;
///
/// case TRUSTLINE:
///     struct
///     {
///         AccountID accountID;
///         TrustLineAsset asset;
///     } trustLine;
///
/// case OFFER:
///     struct
///     {
///         AccountID sellerID;
///         int64 offerID;
///     } offer;
///
/// case DATA:
///     struct
///     {
///         AccountID accountID;
///         string64 dataName;
///     } data;
///
/// case CLAIMABLE_BALANCE:
///     struct
///     {
///         ClaimableBalanceID balanceID;
///     } claimableBalance;
///
/// case LIQUIDITY_POOL:
///     struct
///     {
///         PoolID liquidityPoolID;
///     } liquidityPool;
/// case CONTRACT_DATA:
///     struct
///     {
///         SCAddress contract;
///         SCVal key;
///         ContractDataDurability durability;
///     } contractData;
/// case CONTRACT_CODE:
///     struct
///     {
///         Hash hash;
///     } contractCode;
/// case CONFIG_SETTING:
///     struct
///     {
///         ConfigSettingID configSettingID;
///     } configSetting;
/// case TTL:
///     struct
///     {
///         // Hash of the LedgerKey that is associated with this TTLEntry
///         Hash keyHash;
///     } ttl;
/// };
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
pub enum LedgerKey {
    Account(LedgerKeyAccount),
    Trustline(LedgerKeyTrustLine),
    Offer(LedgerKeyOffer),
    Data(LedgerKeyData),
    ClaimableBalance(LedgerKeyClaimableBalance),
    LiquidityPool(LedgerKeyLiquidityPool),
    ContractData(LedgerKeyContractData),
    ContractCode(LedgerKeyContractCode),
    ConfigSetting(LedgerKeyConfigSetting),
    Ttl(LedgerKeyTtl),
}

#[cfg(feature = "alloc")]
impl Default for LedgerKey {
    fn default() -> Self {
        Self::Account(LedgerKeyAccount::default())
    }
}

impl LedgerKey {
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

impl Name for LedgerKey {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<LedgerEntryType> for LedgerKey {
    #[must_use]
    fn discriminant(&self) -> LedgerEntryType {
        Self::discriminant(self)
    }
}

impl Variants<LedgerEntryType> for LedgerKey {
    fn variants() -> slice::Iter<'static, LedgerEntryType> {
        Self::VARIANTS.iter()
    }
}

impl Union<LedgerEntryType> for LedgerKey {}

impl ReadXdr for LedgerKey {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: LedgerEntryType = <LedgerEntryType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                LedgerEntryType::Account => Self::Account(LedgerKeyAccount::read_xdr(r)?),
                LedgerEntryType::Trustline => Self::Trustline(LedgerKeyTrustLine::read_xdr(r)?),
                LedgerEntryType::Offer => Self::Offer(LedgerKeyOffer::read_xdr(r)?),
                LedgerEntryType::Data => Self::Data(LedgerKeyData::read_xdr(r)?),
                LedgerEntryType::ClaimableBalance => {
                    Self::ClaimableBalance(LedgerKeyClaimableBalance::read_xdr(r)?)
                }
                LedgerEntryType::LiquidityPool => {
                    Self::LiquidityPool(LedgerKeyLiquidityPool::read_xdr(r)?)
                }
                LedgerEntryType::ContractData => {
                    Self::ContractData(LedgerKeyContractData::read_xdr(r)?)
                }
                LedgerEntryType::ContractCode => {
                    Self::ContractCode(LedgerKeyContractCode::read_xdr(r)?)
                }
                LedgerEntryType::ConfigSetting => {
                    Self::ConfigSetting(LedgerKeyConfigSetting::read_xdr(r)?)
                }
                LedgerEntryType::Ttl => Self::Ttl(LedgerKeyTtl::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for LedgerKey {
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
/// Lazy wrapper for [`LedgerKey`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyLedgerKey(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyLedgerKey {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyLedgerKey {
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
impl LazyXdr for LazyLedgerKey {
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
                    <LazyLedgerKeyAccount as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            1 => {
                let field_len =
                    <LazyLedgerKeyTrustLine as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            2 => {
                let field_len =
                    <LazyLedgerKeyOffer as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            3 => {
                let field_len =
                    <LazyLedgerKeyData as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            4 => {
                let field_len = <LazyLedgerKeyClaimableBalance as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            5 => {
                let field_len = <LazyLedgerKeyLiquidityPool as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            6 => {
                let field_len = <LazyLedgerKeyContractData as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            7 => {
                let field_len = <LazyLedgerKeyContractCode as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            8 => {
                let field_len = <LazyLedgerKeyConfigSetting as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            9 => {
                let field_len =
                    <LazyLedgerKeyTtl as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
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
                pos += <LazyLedgerKeyAccount as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            1 => {
                pos += <LazyLedgerKeyTrustLine as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            2 => {
                pos += <LazyLedgerKeyOffer as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            3 => {
                pos += <LazyLedgerKeyData as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            4 => {
                pos += <LazyLedgerKeyClaimableBalance as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            5 => {
                pos += <LazyLedgerKeyLiquidityPool as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            6 => {
                pos += <LazyLedgerKeyContractData as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            7 => {
                pos += <LazyLedgerKeyContractCode as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            8 => {
                pos += <LazyLedgerKeyConfigSetting as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            9 => {
                pos += <LazyLedgerKeyTtl as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyLedgerKey {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyLedgerKey {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyLedgerKey {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyLedgerKey {
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
    pub fn as_account(&self) -> Option<LazyLedgerKeyAccount> {
        if self.discriminant_i32() == 0 {
            Some(<LazyLedgerKeyAccount as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Trustline`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_trustline(&self) -> Option<LazyLedgerKeyTrustLine> {
        if self.discriminant_i32() == 1 {
            Some(<LazyLedgerKeyTrustLine as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Offer`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_offer(&self) -> Option<LazyLedgerKeyOffer> {
        if self.discriminant_i32() == 2 {
            Some(<LazyLedgerKeyOffer as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Data`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_data(&self) -> Option<LazyLedgerKeyData> {
        if self.discriminant_i32() == 3 {
            Some(<LazyLedgerKeyData as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ClaimableBalance`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_claimable_balance(&self) -> Option<LazyLedgerKeyClaimableBalance> {
        if self.discriminant_i32() == 4 {
            Some(<LazyLedgerKeyClaimableBalance as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `LiquidityPool`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_liquidity_pool(&self) -> Option<LazyLedgerKeyLiquidityPool> {
        if self.discriminant_i32() == 5 {
            Some(<LazyLedgerKeyLiquidityPool as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `ContractData`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_contract_data(&self) -> Option<LazyLedgerKeyContractData> {
        if self.discriminant_i32() == 6 {
            Some(<LazyLedgerKeyContractData as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `ContractCode`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_contract_code(&self) -> Option<LazyLedgerKeyContractCode> {
        if self.discriminant_i32() == 7 {
            Some(<LazyLedgerKeyContractCode as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `ConfigSetting`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_config_setting(&self) -> Option<LazyLedgerKeyConfigSetting> {
        if self.discriminant_i32() == 8 {
            Some(<LazyLedgerKeyConfigSetting as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `Ttl`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_ttl(&self) -> Option<LazyLedgerKeyTtl> {
        if self.discriminant_i32() == 9 {
            Some(<LazyLedgerKeyTtl as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LedgerKey> for LazyLedgerKey {
    type Error = Error;
    fn try_from(val: &LedgerKey) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyLedgerKey> for LedgerKey {
    type Error = Error;
    fn try_from(lazy: &LazyLedgerKey) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
