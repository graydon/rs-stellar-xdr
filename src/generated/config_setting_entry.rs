#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ConfigSettingEntry is an XDR Union defined as:
///
/// ```text
/// union ConfigSettingEntry switch (ConfigSettingID configSettingID)
/// {
/// case CONFIG_SETTING_CONTRACT_MAX_SIZE_BYTES:
///     uint32 contractMaxSizeBytes;
/// case CONFIG_SETTING_CONTRACT_COMPUTE_V0:
///     ConfigSettingContractComputeV0 contractCompute;
/// case CONFIG_SETTING_CONTRACT_LEDGER_COST_V0:
///     ConfigSettingContractLedgerCostV0 contractLedgerCost;
/// case CONFIG_SETTING_CONTRACT_HISTORICAL_DATA_V0:
///     ConfigSettingContractHistoricalDataV0 contractHistoricalData;
/// case CONFIG_SETTING_CONTRACT_EVENTS_V0:
///     ConfigSettingContractEventsV0 contractEvents;
/// case CONFIG_SETTING_CONTRACT_BANDWIDTH_V0:
///     ConfigSettingContractBandwidthV0 contractBandwidth;
/// case CONFIG_SETTING_CONTRACT_COST_PARAMS_CPU_INSTRUCTIONS:
///     ContractCostParams contractCostParamsCpuInsns;
/// case CONFIG_SETTING_CONTRACT_COST_PARAMS_MEMORY_BYTES:
///     ContractCostParams contractCostParamsMemBytes;
/// case CONFIG_SETTING_CONTRACT_DATA_KEY_SIZE_BYTES:
///     uint32 contractDataKeySizeBytes;
/// case CONFIG_SETTING_CONTRACT_DATA_ENTRY_SIZE_BYTES:
///     uint32 contractDataEntrySizeBytes;
/// case CONFIG_SETTING_STATE_ARCHIVAL:
///     StateArchivalSettings stateArchivalSettings;
/// case CONFIG_SETTING_CONTRACT_EXECUTION_LANES:
///     ConfigSettingContractExecutionLanesV0 contractExecutionLanes;
/// case CONFIG_SETTING_LIVE_SOROBAN_STATE_SIZE_WINDOW:
///     uint64 liveSorobanStateSizeWindow<>;
/// case CONFIG_SETTING_EVICTION_ITERATOR:
///     EvictionIterator evictionIterator;
/// case CONFIG_SETTING_CONTRACT_PARALLEL_COMPUTE_V0:
///     ConfigSettingContractParallelComputeV0 contractParallelCompute;
/// case CONFIG_SETTING_CONTRACT_LEDGER_COST_EXT_V0:
///     ConfigSettingContractLedgerCostExtV0 contractLedgerCostExt;
/// case CONFIG_SETTING_SCP_TIMING:
///     ConfigSettingSCPTiming contractSCPTiming;
/// case CONFIG_SETTING_FROZEN_LEDGER_KEYS:
///     FrozenLedgerKeys frozenLedgerKeys;
/// case CONFIG_SETTING_FROZEN_LEDGER_KEYS_DELTA:
///     FrozenLedgerKeysDelta frozenLedgerKeysDelta;
/// case CONFIG_SETTING_FREEZE_BYPASS_TXS:
///     FreezeBypassTxs freezeBypassTxs;
/// case CONFIG_SETTING_FREEZE_BYPASS_TXS_DELTA:
///     FreezeBypassTxsDelta freezeBypassTxsDelta;
/// };
/// ```
///
// union with discriminant ConfigSettingId
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
pub enum ConfigSettingEntry {
    ContractMaxSizeBytes(u32),
    ContractComputeV0(ConfigSettingContractComputeV0),
    ContractLedgerCostV0(ConfigSettingContractLedgerCostV0),
    ContractHistoricalDataV0(ConfigSettingContractHistoricalDataV0),
    ContractEventsV0(ConfigSettingContractEventsV0),
    ContractBandwidthV0(ConfigSettingContractBandwidthV0),
    ContractCostParamsCpuInstructions(ContractCostParams),
    ContractCostParamsMemoryBytes(ContractCostParams),
    ContractDataKeySizeBytes(u32),
    ContractDataEntrySizeBytes(u32),
    StateArchival(StateArchivalSettings),
    ContractExecutionLanes(ConfigSettingContractExecutionLanesV0),
    LiveSorobanStateSizeWindow(
        #[cfg_attr(
            all(feature = "serde", feature = "alloc"),
            serde_as(as = "VecM<NumberOrString>")
        )]
        VecM<u64>,
    ),
    EvictionIterator(EvictionIterator),
    ContractParallelComputeV0(ConfigSettingContractParallelComputeV0),
    ContractLedgerCostExtV0(ConfigSettingContractLedgerCostExtV0),
    ScpTiming(ConfigSettingScpTiming),
    FrozenLedgerKeys(FrozenLedgerKeys),
    FrozenLedgerKeysDelta(FrozenLedgerKeysDelta),
    FreezeBypassTxs(FreezeBypassTxs),
    FreezeBypassTxsDelta(FreezeBypassTxsDelta),
}

#[cfg(feature = "alloc")]
impl Default for ConfigSettingEntry {
    fn default() -> Self {
        Self::ContractMaxSizeBytes(u32::default())
    }
}

impl ConfigSettingEntry {
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
            Self::ContractMaxSizeBytes(_) => "ContractMaxSizeBytes",
            Self::ContractComputeV0(_) => "ContractComputeV0",
            Self::ContractLedgerCostV0(_) => "ContractLedgerCostV0",
            Self::ContractHistoricalDataV0(_) => "ContractHistoricalDataV0",
            Self::ContractEventsV0(_) => "ContractEventsV0",
            Self::ContractBandwidthV0(_) => "ContractBandwidthV0",
            Self::ContractCostParamsCpuInstructions(_) => "ContractCostParamsCpuInstructions",
            Self::ContractCostParamsMemoryBytes(_) => "ContractCostParamsMemoryBytes",
            Self::ContractDataKeySizeBytes(_) => "ContractDataKeySizeBytes",
            Self::ContractDataEntrySizeBytes(_) => "ContractDataEntrySizeBytes",
            Self::StateArchival(_) => "StateArchival",
            Self::ContractExecutionLanes(_) => "ContractExecutionLanes",
            Self::LiveSorobanStateSizeWindow(_) => "LiveSorobanStateSizeWindow",
            Self::EvictionIterator(_) => "EvictionIterator",
            Self::ContractParallelComputeV0(_) => "ContractParallelComputeV0",
            Self::ContractLedgerCostExtV0(_) => "ContractLedgerCostExtV0",
            Self::ScpTiming(_) => "ScpTiming",
            Self::FrozenLedgerKeys(_) => "FrozenLedgerKeys",
            Self::FrozenLedgerKeysDelta(_) => "FrozenLedgerKeysDelta",
            Self::FreezeBypassTxs(_) => "FreezeBypassTxs",
            Self::FreezeBypassTxsDelta(_) => "FreezeBypassTxsDelta",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ConfigSettingId {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ContractMaxSizeBytes(_) => ConfigSettingId::ContractMaxSizeBytes,
            Self::ContractComputeV0(_) => ConfigSettingId::ContractComputeV0,
            Self::ContractLedgerCostV0(_) => ConfigSettingId::ContractLedgerCostV0,
            Self::ContractHistoricalDataV0(_) => ConfigSettingId::ContractHistoricalDataV0,
            Self::ContractEventsV0(_) => ConfigSettingId::ContractEventsV0,
            Self::ContractBandwidthV0(_) => ConfigSettingId::ContractBandwidthV0,
            Self::ContractCostParamsCpuInstructions(_) => {
                ConfigSettingId::ContractCostParamsCpuInstructions
            }
            Self::ContractCostParamsMemoryBytes(_) => {
                ConfigSettingId::ContractCostParamsMemoryBytes
            }
            Self::ContractDataKeySizeBytes(_) => ConfigSettingId::ContractDataKeySizeBytes,
            Self::ContractDataEntrySizeBytes(_) => ConfigSettingId::ContractDataEntrySizeBytes,
            Self::StateArchival(_) => ConfigSettingId::StateArchival,
            Self::ContractExecutionLanes(_) => ConfigSettingId::ContractExecutionLanes,
            Self::LiveSorobanStateSizeWindow(_) => ConfigSettingId::LiveSorobanStateSizeWindow,
            Self::EvictionIterator(_) => ConfigSettingId::EvictionIterator,
            Self::ContractParallelComputeV0(_) => ConfigSettingId::ContractParallelComputeV0,
            Self::ContractLedgerCostExtV0(_) => ConfigSettingId::ContractLedgerCostExtV0,
            Self::ScpTiming(_) => ConfigSettingId::ScpTiming,
            Self::FrozenLedgerKeys(_) => ConfigSettingId::FrozenLedgerKeys,
            Self::FrozenLedgerKeysDelta(_) => ConfigSettingId::FrozenLedgerKeysDelta,
            Self::FreezeBypassTxs(_) => ConfigSettingId::FreezeBypassTxs,
            Self::FreezeBypassTxsDelta(_) => ConfigSettingId::FreezeBypassTxsDelta,
        }
    }

    #[must_use]
    pub const fn variants() -> [ConfigSettingId; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ConfigSettingEntry {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ConfigSettingId> for ConfigSettingEntry {
    #[must_use]
    fn discriminant(&self) -> ConfigSettingId {
        Self::discriminant(self)
    }
}

impl Variants<ConfigSettingId> for ConfigSettingEntry {
    fn variants() -> slice::Iter<'static, ConfigSettingId> {
        Self::VARIANTS.iter()
    }
}

impl Union<ConfigSettingId> for ConfigSettingEntry {}

impl ReadXdr for ConfigSettingEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ConfigSettingId = <ConfigSettingId as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ConfigSettingId::ContractMaxSizeBytes => {
                    Self::ContractMaxSizeBytes(u32::read_xdr(r)?)
                }
                ConfigSettingId::ContractComputeV0 => {
                    Self::ContractComputeV0(ConfigSettingContractComputeV0::read_xdr(r)?)
                }
                ConfigSettingId::ContractLedgerCostV0 => {
                    Self::ContractLedgerCostV0(ConfigSettingContractLedgerCostV0::read_xdr(r)?)
                }
                ConfigSettingId::ContractHistoricalDataV0 => Self::ContractHistoricalDataV0(
                    ConfigSettingContractHistoricalDataV0::read_xdr(r)?,
                ),
                ConfigSettingId::ContractEventsV0 => {
                    Self::ContractEventsV0(ConfigSettingContractEventsV0::read_xdr(r)?)
                }
                ConfigSettingId::ContractBandwidthV0 => {
                    Self::ContractBandwidthV0(ConfigSettingContractBandwidthV0::read_xdr(r)?)
                }
                ConfigSettingId::ContractCostParamsCpuInstructions => {
                    Self::ContractCostParamsCpuInstructions(ContractCostParams::read_xdr(r)?)
                }
                ConfigSettingId::ContractCostParamsMemoryBytes => {
                    Self::ContractCostParamsMemoryBytes(ContractCostParams::read_xdr(r)?)
                }
                ConfigSettingId::ContractDataKeySizeBytes => {
                    Self::ContractDataKeySizeBytes(u32::read_xdr(r)?)
                }
                ConfigSettingId::ContractDataEntrySizeBytes => {
                    Self::ContractDataEntrySizeBytes(u32::read_xdr(r)?)
                }
                ConfigSettingId::StateArchival => {
                    Self::StateArchival(StateArchivalSettings::read_xdr(r)?)
                }
                ConfigSettingId::ContractExecutionLanes => Self::ContractExecutionLanes(
                    ConfigSettingContractExecutionLanesV0::read_xdr(r)?,
                ),
                ConfigSettingId::LiveSorobanStateSizeWindow => {
                    Self::LiveSorobanStateSizeWindow(VecM::<u64>::read_xdr(r)?)
                }
                ConfigSettingId::EvictionIterator => {
                    Self::EvictionIterator(EvictionIterator::read_xdr(r)?)
                }
                ConfigSettingId::ContractParallelComputeV0 => Self::ContractParallelComputeV0(
                    ConfigSettingContractParallelComputeV0::read_xdr(r)?,
                ),
                ConfigSettingId::ContractLedgerCostExtV0 => Self::ContractLedgerCostExtV0(
                    ConfigSettingContractLedgerCostExtV0::read_xdr(r)?,
                ),
                ConfigSettingId::ScpTiming => Self::ScpTiming(ConfigSettingScpTiming::read_xdr(r)?),
                ConfigSettingId::FrozenLedgerKeys => {
                    Self::FrozenLedgerKeys(FrozenLedgerKeys::read_xdr(r)?)
                }
                ConfigSettingId::FrozenLedgerKeysDelta => {
                    Self::FrozenLedgerKeysDelta(FrozenLedgerKeysDelta::read_xdr(r)?)
                }
                ConfigSettingId::FreezeBypassTxs => {
                    Self::FreezeBypassTxs(FreezeBypassTxs::read_xdr(r)?)
                }
                ConfigSettingId::FreezeBypassTxsDelta => {
                    Self::FreezeBypassTxsDelta(FreezeBypassTxsDelta::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ConfigSettingEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::ContractMaxSizeBytes(v) => v.write_xdr(w)?,
                Self::ContractComputeV0(v) => v.write_xdr(w)?,
                Self::ContractLedgerCostV0(v) => v.write_xdr(w)?,
                Self::ContractHistoricalDataV0(v) => v.write_xdr(w)?,
                Self::ContractEventsV0(v) => v.write_xdr(w)?,
                Self::ContractBandwidthV0(v) => v.write_xdr(w)?,
                Self::ContractCostParamsCpuInstructions(v) => v.write_xdr(w)?,
                Self::ContractCostParamsMemoryBytes(v) => v.write_xdr(w)?,
                Self::ContractDataKeySizeBytes(v) => v.write_xdr(w)?,
                Self::ContractDataEntrySizeBytes(v) => v.write_xdr(w)?,
                Self::StateArchival(v) => v.write_xdr(w)?,
                Self::ContractExecutionLanes(v) => v.write_xdr(w)?,
                Self::LiveSorobanStateSizeWindow(v) => v.write_xdr(w)?,
                Self::EvictionIterator(v) => v.write_xdr(w)?,
                Self::ContractParallelComputeV0(v) => v.write_xdr(w)?,
                Self::ContractLedgerCostExtV0(v) => v.write_xdr(w)?,
                Self::ScpTiming(v) => v.write_xdr(w)?,
                Self::FrozenLedgerKeys(v) => v.write_xdr(w)?,
                Self::FrozenLedgerKeysDelta(v) => v.write_xdr(w)?,
                Self::FreezeBypassTxs(v) => v.write_xdr(w)?,
                Self::FreezeBypassTxsDelta(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ConfigSettingEntry`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyConfigSettingEntry(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyConfigSettingEntry {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyConfigSettingEntry {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self
                .as_contract_max_size_bytes()
                .cmp(&other.as_contract_max_size_bytes()),
            1 => self
                .as_contract_compute_v0()
                .cmp(&other.as_contract_compute_v0()),
            2 => self
                .as_contract_ledger_cost_v0()
                .cmp(&other.as_contract_ledger_cost_v0()),
            3 => self
                .as_contract_historical_data_v0()
                .cmp(&other.as_contract_historical_data_v0()),
            4 => self
                .as_contract_events_v0()
                .cmp(&other.as_contract_events_v0()),
            5 => self
                .as_contract_bandwidth_v0()
                .cmp(&other.as_contract_bandwidth_v0()),
            6 => self
                .as_contract_cost_params_cpu_instructions()
                .cmp(&other.as_contract_cost_params_cpu_instructions()),
            7 => self
                .as_contract_cost_params_memory_bytes()
                .cmp(&other.as_contract_cost_params_memory_bytes()),
            8 => self
                .as_contract_data_key_size_bytes()
                .cmp(&other.as_contract_data_key_size_bytes()),
            9 => self
                .as_contract_data_entry_size_bytes()
                .cmp(&other.as_contract_data_entry_size_bytes()),
            10 => self.as_state_archival().cmp(&other.as_state_archival()),
            11 => self
                .as_contract_execution_lanes()
                .cmp(&other.as_contract_execution_lanes()),
            12 => self
                .as_live_soroban_state_size_window()
                .cmp(&other.as_live_soroban_state_size_window()),
            13 => self
                .as_eviction_iterator()
                .cmp(&other.as_eviction_iterator()),
            14 => self
                .as_contract_parallel_compute_v0()
                .cmp(&other.as_contract_parallel_compute_v0()),
            15 => self
                .as_contract_ledger_cost_ext_v0()
                .cmp(&other.as_contract_ledger_cost_ext_v0()),
            16 => self.as_scp_timing().cmp(&other.as_scp_timing()),
            17 => self
                .as_frozen_ledger_keys()
                .cmp(&other.as_frozen_ledger_keys()),
            18 => self
                .as_frozen_ledger_keys_delta()
                .cmp(&other.as_frozen_ledger_keys_delta()),
            19 => self
                .as_freeze_bypass_txs()
                .cmp(&other.as_freeze_bypass_txs()),
            20 => self
                .as_freeze_bypass_txs_delta()
                .cmp(&other.as_freeze_bypass_txs_delta()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyConfigSettingEntry {
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
            0 => {
                let field_len = <u32 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            1 => {
                let field_len = <LazyConfigSettingContractComputeV0 as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            2 => {
                let field_len = <LazyConfigSettingContractLedgerCostV0 as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            3 => {
                let field_len =
                    <LazyConfigSettingContractHistoricalDataV0 as LazyXdr>::xdr_validate(
                        &buf[pos as usize..],
                        depth,
                    )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            4 => {
                let field_len = <LazyConfigSettingContractEventsV0 as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            5 => {
                let field_len = <LazyConfigSettingContractBandwidthV0 as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            6 => {
                let field_len =
                    <LazyContractCostParams as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            7 => {
                let field_len =
                    <LazyContractCostParams as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            8 => {
                let field_len = <u32 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            9 => {
                let field_len = <u32 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            10 => {
                let field_len = <LazyStateArchivalSettings as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            11 => {
                let field_len =
                    <LazyConfigSettingContractExecutionLanesV0 as LazyXdr>::xdr_validate(
                        &buf[pos as usize..],
                        depth,
                    )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            12 => {
                let field_len =
                    <LazyVecM<u64> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            13 => {
                let field_len =
                    <LazyEvictionIterator as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            14 => {
                let field_len =
                    <LazyConfigSettingContractParallelComputeV0 as LazyXdr>::xdr_validate(
                        &buf[pos as usize..],
                        depth,
                    )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            15 => {
                let field_len =
                    <LazyConfigSettingContractLedgerCostExtV0 as LazyXdr>::xdr_validate(
                        &buf[pos as usize..],
                        depth,
                    )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            16 => {
                let field_len = <LazyConfigSettingScpTiming as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            17 => {
                let field_len =
                    <LazyFrozenLedgerKeys as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            18 => {
                let field_len = <LazyFrozenLedgerKeysDelta as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            19 => {
                let field_len =
                    <LazyFreezeBypassTxs as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            20 => {
                let field_len = <LazyFreezeBypassTxsDelta as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
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
            0 => {
                pos += <u32 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            1 => {
                pos +=
                    <LazyConfigSettingContractComputeV0 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            2 => {
                pos += <LazyConfigSettingContractLedgerCostV0 as LazyXdr>::xdr_len(
                    &buf[pos as usize..],
                );
            }
            3 => {
                pos += <LazyConfigSettingContractHistoricalDataV0 as LazyXdr>::xdr_len(
                    &buf[pos as usize..],
                );
            }
            4 => {
                pos +=
                    <LazyConfigSettingContractEventsV0 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            5 => {
                pos += <LazyConfigSettingContractBandwidthV0 as LazyXdr>::xdr_len(
                    &buf[pos as usize..],
                );
            }
            6 => {
                pos += <LazyContractCostParams as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            7 => {
                pos += <LazyContractCostParams as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            8 => {
                pos += <u32 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            9 => {
                pos += <u32 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            10 => {
                pos += <LazyStateArchivalSettings as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            11 => {
                pos += <LazyConfigSettingContractExecutionLanesV0 as LazyXdr>::xdr_len(
                    &buf[pos as usize..],
                );
            }
            12 => {
                pos += <LazyVecM<u64> as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            13 => {
                pos += <LazyEvictionIterator as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            14 => {
                pos += <LazyConfigSettingContractParallelComputeV0 as LazyXdr>::xdr_len(
                    &buf[pos as usize..],
                );
            }
            15 => {
                pos += <LazyConfigSettingContractLedgerCostExtV0 as LazyXdr>::xdr_len(
                    &buf[pos as usize..],
                );
            }
            16 => {
                pos += <LazyConfigSettingScpTiming as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            17 => {
                pos += <LazyFrozenLedgerKeys as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            18 => {
                pos += <LazyFrozenLedgerKeysDelta as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            19 => {
                pos += <LazyFreezeBypassTxs as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            20 => {
                pos += <LazyFreezeBypassTxsDelta as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyConfigSettingEntry {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyConfigSettingEntry {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyConfigSettingEntry {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyConfigSettingEntry {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> ConfigSettingId {
        // Validated — unwrap is safe.
        ConfigSettingId::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `ContractMaxSizeBytes`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_contract_max_size_bytes(&self) -> Option<u32> {
        if self.discriminant_i32() == 0 {
            Some(<u32 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ContractComputeV0`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_contract_compute_v0(&self) -> Option<LazyConfigSettingContractComputeV0> {
        if self.discriminant_i32() == 1 {
            Some(<LazyConfigSettingContractComputeV0 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ContractLedgerCostV0`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_contract_ledger_cost_v0(&self) -> Option<LazyConfigSettingContractLedgerCostV0> {
        if self.discriminant_i32() == 2 {
            Some(<LazyConfigSettingContractLedgerCostV0 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ContractHistoricalDataV0`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_contract_historical_data_v0(
        &self,
    ) -> Option<LazyConfigSettingContractHistoricalDataV0> {
        if self.discriminant_i32() == 3 {
            Some(<LazyConfigSettingContractHistoricalDataV0 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ContractEventsV0`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_contract_events_v0(&self) -> Option<LazyConfigSettingContractEventsV0> {
        if self.discriminant_i32() == 4 {
            Some(<LazyConfigSettingContractEventsV0 as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `ContractBandwidthV0`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_contract_bandwidth_v0(&self) -> Option<LazyConfigSettingContractBandwidthV0> {
        if self.discriminant_i32() == 5 {
            Some(<LazyConfigSettingContractBandwidthV0 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ContractCostParamsCpuInstructions`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_contract_cost_params_cpu_instructions(&self) -> Option<LazyContractCostParams> {
        if self.discriminant_i32() == 6 {
            Some(<LazyContractCostParams as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ContractCostParamsMemoryBytes`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_contract_cost_params_memory_bytes(&self) -> Option<LazyContractCostParams> {
        if self.discriminant_i32() == 7 {
            Some(<LazyContractCostParams as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ContractDataKeySizeBytes`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_contract_data_key_size_bytes(&self) -> Option<u32> {
        if self.discriminant_i32() == 8 {
            Some(<u32 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ContractDataEntrySizeBytes`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_contract_data_entry_size_bytes(&self) -> Option<u32> {
        if self.discriminant_i32() == 9 {
            Some(<u32 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `StateArchival`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_state_archival(&self) -> Option<LazyStateArchivalSettings> {
        if self.discriminant_i32() == 10 {
            Some(<LazyStateArchivalSettings as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `ContractExecutionLanes`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_contract_execution_lanes(&self) -> Option<LazyConfigSettingContractExecutionLanesV0> {
        if self.discriminant_i32() == 11 {
            Some(<LazyConfigSettingContractExecutionLanesV0 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `LiveSorobanStateSizeWindow`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_live_soroban_state_size_window(&self) -> Option<LazyVecM<u64>> {
        if self.discriminant_i32() == 12 {
            Some(<LazyVecM<u64> as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `EvictionIterator`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_eviction_iterator(&self) -> Option<LazyEvictionIterator> {
        if self.discriminant_i32() == 13 {
            Some(<LazyEvictionIterator as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ContractParallelComputeV0`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_contract_parallel_compute_v0(
        &self,
    ) -> Option<LazyConfigSettingContractParallelComputeV0> {
        if self.discriminant_i32() == 14 {
            Some(<LazyConfigSettingContractParallelComputeV0 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ContractLedgerCostExtV0`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_contract_ledger_cost_ext_v0(
        &self,
    ) -> Option<LazyConfigSettingContractLedgerCostExtV0> {
        if self.discriminant_i32() == 15 {
            Some(<LazyConfigSettingContractLedgerCostExtV0 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ScpTiming`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_scp_timing(&self) -> Option<LazyConfigSettingScpTiming> {
        if self.discriminant_i32() == 16 {
            Some(<LazyConfigSettingScpTiming as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `FrozenLedgerKeys`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_frozen_ledger_keys(&self) -> Option<LazyFrozenLedgerKeys> {
        if self.discriminant_i32() == 17 {
            Some(<LazyFrozenLedgerKeys as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `FrozenLedgerKeysDelta`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_frozen_ledger_keys_delta(&self) -> Option<LazyFrozenLedgerKeysDelta> {
        if self.discriminant_i32() == 18 {
            Some(<LazyFrozenLedgerKeysDelta as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `FreezeBypassTxs`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_freeze_bypass_txs(&self) -> Option<LazyFreezeBypassTxs> {
        if self.discriminant_i32() == 19 {
            Some(<LazyFreezeBypassTxs as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `FreezeBypassTxsDelta`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_freeze_bypass_txs_delta(&self) -> Option<LazyFreezeBypassTxsDelta> {
        if self.discriminant_i32() == 20 {
            Some(<LazyFreezeBypassTxsDelta as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ConfigSettingEntry> for LazyConfigSettingEntry {
    type Error = Error;
    fn try_from(val: &ConfigSettingEntry) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyConfigSettingEntry> for ConfigSettingEntry {
    type Error = Error;
    fn try_from(lazy: &LazyConfigSettingEntry) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
