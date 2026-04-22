#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ConfigSettingId is an XDR Enum defined as:
///
/// ```text
/// enum ConfigSettingID
/// {
///     CONFIG_SETTING_CONTRACT_MAX_SIZE_BYTES = 0,
///     CONFIG_SETTING_CONTRACT_COMPUTE_V0 = 1,
///     CONFIG_SETTING_CONTRACT_LEDGER_COST_V0 = 2,
///     CONFIG_SETTING_CONTRACT_HISTORICAL_DATA_V0 = 3,
///     CONFIG_SETTING_CONTRACT_EVENTS_V0 = 4,
///     CONFIG_SETTING_CONTRACT_BANDWIDTH_V0 = 5,
///     CONFIG_SETTING_CONTRACT_COST_PARAMS_CPU_INSTRUCTIONS = 6,
///     CONFIG_SETTING_CONTRACT_COST_PARAMS_MEMORY_BYTES = 7,
///     CONFIG_SETTING_CONTRACT_DATA_KEY_SIZE_BYTES = 8,
///     CONFIG_SETTING_CONTRACT_DATA_ENTRY_SIZE_BYTES = 9,
///     CONFIG_SETTING_STATE_ARCHIVAL = 10,
///     CONFIG_SETTING_CONTRACT_EXECUTION_LANES = 11,
///     CONFIG_SETTING_LIVE_SOROBAN_STATE_SIZE_WINDOW = 12,
///     CONFIG_SETTING_EVICTION_ITERATOR = 13,
///     CONFIG_SETTING_CONTRACT_PARALLEL_COMPUTE_V0 = 14,
///     CONFIG_SETTING_CONTRACT_LEDGER_COST_EXT_V0 = 15,
///     CONFIG_SETTING_SCP_TIMING = 16,
///     CONFIG_SETTING_FROZEN_LEDGER_KEYS = 17,
///     CONFIG_SETTING_FROZEN_LEDGER_KEYS_DELTA = 18,
///     CONFIG_SETTING_FREEZE_BYPASS_TXS = 19,
///     CONFIG_SETTING_FREEZE_BYPASS_TXS_DELTA = 20
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
pub enum ConfigSettingId {
    #[cfg_attr(feature = "alloc", default)]
    ContractMaxSizeBytes = 0,
    ContractComputeV0 = 1,
    ContractLedgerCostV0 = 2,
    ContractHistoricalDataV0 = 3,
    ContractEventsV0 = 4,
    ContractBandwidthV0 = 5,
    ContractCostParamsCpuInstructions = 6,
    ContractCostParamsMemoryBytes = 7,
    ContractDataKeySizeBytes = 8,
    ContractDataEntrySizeBytes = 9,
    StateArchival = 10,
    ContractExecutionLanes = 11,
    LiveSorobanStateSizeWindow = 12,
    EvictionIterator = 13,
    ContractParallelComputeV0 = 14,
    ContractLedgerCostExtV0 = 15,
    ScpTiming = 16,
    FrozenLedgerKeys = 17,
    FrozenLedgerKeysDelta = 18,
    FreezeBypassTxs = 19,
    FreezeBypassTxsDelta = 20,
}

impl ConfigSettingId {
    const _VARIANTS: &[ConfigSettingId] = &[
        ConfigSettingId::ContractMaxSizeBytes,
        ConfigSettingId::ContractComputeV0,
        ConfigSettingId::ContractLedgerCostV0,
        ConfigSettingId::ContractHistoricalDataV0,
        ConfigSettingId::ContractEventsV0,
        ConfigSettingId::ContractBandwidthV0,
        ConfigSettingId::ContractCostParamsCpuInstructions,
        ConfigSettingId::ContractCostParamsMemoryBytes,
        ConfigSettingId::ContractDataKeySizeBytes,
        ConfigSettingId::ContractDataEntrySizeBytes,
        ConfigSettingId::StateArchival,
        ConfigSettingId::ContractExecutionLanes,
        ConfigSettingId::LiveSorobanStateSizeWindow,
        ConfigSettingId::EvictionIterator,
        ConfigSettingId::ContractParallelComputeV0,
        ConfigSettingId::ContractLedgerCostExtV0,
        ConfigSettingId::ScpTiming,
        ConfigSettingId::FrozenLedgerKeys,
        ConfigSettingId::FrozenLedgerKeysDelta,
        ConfigSettingId::FreezeBypassTxs,
        ConfigSettingId::FreezeBypassTxsDelta,
    ];
    pub const VARIANTS: [ConfigSettingId; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "ContractMaxSizeBytes",
        "ContractComputeV0",
        "ContractLedgerCostV0",
        "ContractHistoricalDataV0",
        "ContractEventsV0",
        "ContractBandwidthV0",
        "ContractCostParamsCpuInstructions",
        "ContractCostParamsMemoryBytes",
        "ContractDataKeySizeBytes",
        "ContractDataEntrySizeBytes",
        "StateArchival",
        "ContractExecutionLanes",
        "LiveSorobanStateSizeWindow",
        "EvictionIterator",
        "ContractParallelComputeV0",
        "ContractLedgerCostExtV0",
        "ScpTiming",
        "FrozenLedgerKeys",
        "FrozenLedgerKeysDelta",
        "FreezeBypassTxs",
        "FreezeBypassTxsDelta",
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
            Self::ContractMaxSizeBytes => "ContractMaxSizeBytes",
            Self::ContractComputeV0 => "ContractComputeV0",
            Self::ContractLedgerCostV0 => "ContractLedgerCostV0",
            Self::ContractHistoricalDataV0 => "ContractHistoricalDataV0",
            Self::ContractEventsV0 => "ContractEventsV0",
            Self::ContractBandwidthV0 => "ContractBandwidthV0",
            Self::ContractCostParamsCpuInstructions => "ContractCostParamsCpuInstructions",
            Self::ContractCostParamsMemoryBytes => "ContractCostParamsMemoryBytes",
            Self::ContractDataKeySizeBytes => "ContractDataKeySizeBytes",
            Self::ContractDataEntrySizeBytes => "ContractDataEntrySizeBytes",
            Self::StateArchival => "StateArchival",
            Self::ContractExecutionLanes => "ContractExecutionLanes",
            Self::LiveSorobanStateSizeWindow => "LiveSorobanStateSizeWindow",
            Self::EvictionIterator => "EvictionIterator",
            Self::ContractParallelComputeV0 => "ContractParallelComputeV0",
            Self::ContractLedgerCostExtV0 => "ContractLedgerCostExtV0",
            Self::ScpTiming => "ScpTiming",
            Self::FrozenLedgerKeys => "FrozenLedgerKeys",
            Self::FrozenLedgerKeysDelta => "FrozenLedgerKeysDelta",
            Self::FreezeBypassTxs => "FreezeBypassTxs",
            Self::FreezeBypassTxsDelta => "FreezeBypassTxsDelta",
        }
    }

    #[must_use]
    pub const fn variants() -> [ConfigSettingId; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ConfigSettingId {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ConfigSettingId> for ConfigSettingId {
    fn variants() -> slice::Iter<'static, ConfigSettingId> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ConfigSettingId {}

impl fmt::Display for ConfigSettingId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ConfigSettingId {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ConfigSettingId::ContractMaxSizeBytes,
            1 => ConfigSettingId::ContractComputeV0,
            2 => ConfigSettingId::ContractLedgerCostV0,
            3 => ConfigSettingId::ContractHistoricalDataV0,
            4 => ConfigSettingId::ContractEventsV0,
            5 => ConfigSettingId::ContractBandwidthV0,
            6 => ConfigSettingId::ContractCostParamsCpuInstructions,
            7 => ConfigSettingId::ContractCostParamsMemoryBytes,
            8 => ConfigSettingId::ContractDataKeySizeBytes,
            9 => ConfigSettingId::ContractDataEntrySizeBytes,
            10 => ConfigSettingId::StateArchival,
            11 => ConfigSettingId::ContractExecutionLanes,
            12 => ConfigSettingId::LiveSorobanStateSizeWindow,
            13 => ConfigSettingId::EvictionIterator,
            14 => ConfigSettingId::ContractParallelComputeV0,
            15 => ConfigSettingId::ContractLedgerCostExtV0,
            16 => ConfigSettingId::ScpTiming,
            17 => ConfigSettingId::FrozenLedgerKeys,
            18 => ConfigSettingId::FrozenLedgerKeysDelta,
            19 => ConfigSettingId::FreezeBypassTxs,
            20 => ConfigSettingId::FreezeBypassTxsDelta,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ConfigSettingId> for i32 {
    #[must_use]
    fn from(e: ConfigSettingId) -> Self {
        e as Self
    }
}

impl ReadXdr for ConfigSettingId {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ConfigSettingId {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}

#[cfg(feature = "alloc")]
// Enum ConfigSettingId: scalar lazy type — impl LazyXdr directly on the enum.
impl LazyXdr for ConfigSettingId {
    const FIXED_XDR_SIZE: Option<u32> = Some(4);

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let v = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let _ = ConfigSettingId::try_from(v)?;
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
        ConfigSettingId::try_from(v).unwrap()
    }
}
