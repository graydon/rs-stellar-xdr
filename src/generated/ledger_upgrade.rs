#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// LedgerUpgrade is an XDR Union defined as:
///
/// ```text
/// union LedgerUpgrade switch (LedgerUpgradeType type)
/// {
/// case LEDGER_UPGRADE_VERSION:
///     uint32 newLedgerVersion; // update ledgerVersion
/// case LEDGER_UPGRADE_BASE_FEE:
///     uint32 newBaseFee; // update baseFee
/// case LEDGER_UPGRADE_MAX_TX_SET_SIZE:
///     uint32 newMaxTxSetSize; // update maxTxSetSize
/// case LEDGER_UPGRADE_BASE_RESERVE:
///     uint32 newBaseReserve; // update baseReserve
/// case LEDGER_UPGRADE_FLAGS:
///     uint32 newFlags; // update flags
/// case LEDGER_UPGRADE_CONFIG:
///     // Update arbitrary `ConfigSetting` entries identified by the key.
///     ConfigUpgradeSetKey newConfig;
/// case LEDGER_UPGRADE_MAX_SOROBAN_TX_SET_SIZE:
///     // Update ConfigSettingContractExecutionLanesV0.ledgerMaxTxCount without
///     // using `LEDGER_UPGRADE_CONFIG`.
///     uint32 newMaxSorobanTxSetSize;
/// };
/// ```
///
// union with discriminant LedgerUpgradeType
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
pub enum LedgerUpgrade {
    Version(u32),
    BaseFee(u32),
    MaxTxSetSize(u32),
    BaseReserve(u32),
    Flags(u32),
    Config(ConfigUpgradeSetKey),
    MaxSorobanTxSetSize(u32),
}

#[cfg(feature = "alloc")]
impl Default for LedgerUpgrade {
    fn default() -> Self {
        Self::Version(u32::default())
    }
}

impl LedgerUpgrade {
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
            Self::Version(_) => "Version",
            Self::BaseFee(_) => "BaseFee",
            Self::MaxTxSetSize(_) => "MaxTxSetSize",
            Self::BaseReserve(_) => "BaseReserve",
            Self::Flags(_) => "Flags",
            Self::Config(_) => "Config",
            Self::MaxSorobanTxSetSize(_) => "MaxSorobanTxSetSize",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> LedgerUpgradeType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Version(_) => LedgerUpgradeType::Version,
            Self::BaseFee(_) => LedgerUpgradeType::BaseFee,
            Self::MaxTxSetSize(_) => LedgerUpgradeType::MaxTxSetSize,
            Self::BaseReserve(_) => LedgerUpgradeType::BaseReserve,
            Self::Flags(_) => LedgerUpgradeType::Flags,
            Self::Config(_) => LedgerUpgradeType::Config,
            Self::MaxSorobanTxSetSize(_) => LedgerUpgradeType::MaxSorobanTxSetSize,
        }
    }

    #[must_use]
    pub const fn variants() -> [LedgerUpgradeType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for LedgerUpgrade {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<LedgerUpgradeType> for LedgerUpgrade {
    #[must_use]
    fn discriminant(&self) -> LedgerUpgradeType {
        Self::discriminant(self)
    }
}

impl Variants<LedgerUpgradeType> for LedgerUpgrade {
    fn variants() -> slice::Iter<'static, LedgerUpgradeType> {
        Self::VARIANTS.iter()
    }
}

impl Union<LedgerUpgradeType> for LedgerUpgrade {}

impl ReadXdr for LedgerUpgrade {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: LedgerUpgradeType = <LedgerUpgradeType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                LedgerUpgradeType::Version => Self::Version(u32::read_xdr(r)?),
                LedgerUpgradeType::BaseFee => Self::BaseFee(u32::read_xdr(r)?),
                LedgerUpgradeType::MaxTxSetSize => Self::MaxTxSetSize(u32::read_xdr(r)?),
                LedgerUpgradeType::BaseReserve => Self::BaseReserve(u32::read_xdr(r)?),
                LedgerUpgradeType::Flags => Self::Flags(u32::read_xdr(r)?),
                LedgerUpgradeType::Config => Self::Config(ConfigUpgradeSetKey::read_xdr(r)?),
                LedgerUpgradeType::MaxSorobanTxSetSize => {
                    Self::MaxSorobanTxSetSize(u32::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for LedgerUpgrade {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Version(v) => v.write_xdr(w)?,
                Self::BaseFee(v) => v.write_xdr(w)?,
                Self::MaxTxSetSize(v) => v.write_xdr(w)?,
                Self::BaseReserve(v) => v.write_xdr(w)?,
                Self::Flags(v) => v.write_xdr(w)?,
                Self::Config(v) => v.write_xdr(w)?,
                Self::MaxSorobanTxSetSize(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`LedgerUpgrade`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyLedgerUpgrade(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyLedgerUpgrade {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyLedgerUpgrade {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            1 => self.as_version().cmp(&other.as_version()),
            2 => self.as_base_fee().cmp(&other.as_base_fee()),
            3 => self.as_max_tx_set_size().cmp(&other.as_max_tx_set_size()),
            4 => self.as_base_reserve().cmp(&other.as_base_reserve()),
            5 => self.as_flags().cmp(&other.as_flags()),
            6 => self.as_config().cmp(&other.as_config()),
            7 => self
                .as_max_soroban_tx_set_size()
                .cmp(&other.as_max_soroban_tx_set_size()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyLedgerUpgrade {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let bytes: [u8; 4] = buf[..4].try_into().unwrap();
        let disc = i32::from_be_bytes(bytes);
        #[allow(unused_mut)]
        let mut pos: u32 = 4;
        #[allow(clippy::match_same_arms)]
        match disc {
            1 => {
                let field_len = <u32 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            2 => {
                let field_len = <u32 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            3 => {
                let field_len = <u32 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            4 => {
                let field_len = <u32 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            5 => {
                let field_len = <u32 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            6 => {
                let field_len = <LazyConfigUpgradeSetKey as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            7 => {
                let field_len = <u32 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            _ => return Err(Error::Invalid),
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let bytes: [u8; 4] = buf[..4].try_into().unwrap();
        let disc = i32::from_be_bytes(bytes);
        #[allow(unused_mut)]
        let mut pos: u32 = 4;
        #[allow(clippy::match_same_arms)]
        match disc {
            1 => {
                pos += <u32 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            2 => {
                pos += <u32 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            3 => {
                pos += <u32 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            4 => {
                pos += <u32 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            5 => {
                pos += <u32 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            6 => {
                pos += <LazyConfigUpgradeSetKey as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            7 => {
                pos += <u32 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            _ => {}
        }
        pos
    }

    fn from_xdr_consume(parent: &LazyHandle, buf: &mut &[u8]) -> Self {
        let len = Self::xdr_len(buf);
        let offset = (parent.len() as usize - buf.len()) as u32;
        let handle = parent.sub_handle(offset, len);
        *buf = &buf[len as usize..];
        Self(handle)
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyLedgerUpgrade {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyLedgerUpgrade {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyLedgerUpgrade {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyLedgerUpgrade {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> LedgerUpgradeType {
        // Validated — unwrap is safe.
        LedgerUpgradeType::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `Version`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_version(&self) -> Option<u32> {
        if self.discriminant_i32() == 1 {
            Some(<u32 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `BaseFee`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_base_fee(&self) -> Option<u32> {
        if self.discriminant_i32() == 2 {
            Some(<u32 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `MaxTxSetSize`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_max_tx_set_size(&self) -> Option<u32> {
        if self.discriminant_i32() == 3 {
            Some(<u32 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `BaseReserve`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_base_reserve(&self) -> Option<u32> {
        if self.discriminant_i32() == 4 {
            Some(<u32 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Flags`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_flags(&self) -> Option<u32> {
        if self.discriminant_i32() == 5 {
            Some(<u32 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Config`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_config(&self) -> Option<LazyConfigUpgradeSetKey> {
        if self.discriminant_i32() == 6 {
            Some(<LazyConfigUpgradeSetKey as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `MaxSorobanTxSetSize`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_max_soroban_tx_set_size(&self) -> Option<u32> {
        if self.discriminant_i32() == 7 {
            Some(<u32 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LedgerUpgrade> for LazyLedgerUpgrade {
    type Error = Error;
    fn try_from(val: &LedgerUpgrade) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyLedgerUpgrade> for LedgerUpgrade {
    type Error = Error;
    fn try_from(lazy: &LazyLedgerUpgrade) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
