#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// StateArchivalSettings is an XDR Struct defined as:
///
/// ```text
/// struct StateArchivalSettings {
///     uint32 maxEntryTTL;
///     uint32 minTemporaryTTL;
///     uint32 minPersistentTTL;
///
///     // rent_fee = wfee_rate_average / rent_rate_denominator_for_type
///     int64 persistentRentRateDenominator;
///     int64 tempRentRateDenominator;
///
///     // max number of entries that emit archival meta in a single ledger
///     uint32 maxEntriesToArchive;
///
///     // Number of snapshots to use when calculating average live Soroban State size
///     uint32 liveSorobanStateSizeWindowSampleSize;
///
///     // How often to sample the live Soroban State size for the average, in ledgers
///     uint32 liveSorobanStateSizeWindowSamplePeriod;
///
///     // Maximum number of bytes that we scan for eviction per ledger
///     uint32 evictionScanSize;
///
///     // Lowest BucketList level to be scanned to evict entries
///     uint32 startingEvictionScanLevel;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct StateArchivalSettings {
    pub max_entry_ttl: u32,
    pub min_temporary_ttl: u32,
    pub min_persistent_ttl: u32,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub persistent_rent_rate_denominator: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub temp_rent_rate_denominator: i64,
    pub max_entries_to_archive: u32,
    pub live_soroban_state_size_window_sample_size: u32,
    pub live_soroban_state_size_window_sample_period: u32,
    pub eviction_scan_size: u32,
    pub starting_eviction_scan_level: u32,
}

impl ReadXdr for StateArchivalSettings {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                max_entry_ttl: u32::read_xdr(r)?,
                min_temporary_ttl: u32::read_xdr(r)?,
                min_persistent_ttl: u32::read_xdr(r)?,
                persistent_rent_rate_denominator: i64::read_xdr(r)?,
                temp_rent_rate_denominator: i64::read_xdr(r)?,
                max_entries_to_archive: u32::read_xdr(r)?,
                live_soroban_state_size_window_sample_size: u32::read_xdr(r)?,
                live_soroban_state_size_window_sample_period: u32::read_xdr(r)?,
                eviction_scan_size: u32::read_xdr(r)?,
                starting_eviction_scan_level: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for StateArchivalSettings {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.max_entry_ttl.write_xdr(w)?;
            self.min_temporary_ttl.write_xdr(w)?;
            self.min_persistent_ttl.write_xdr(w)?;
            self.persistent_rent_rate_denominator.write_xdr(w)?;
            self.temp_rent_rate_denominator.write_xdr(w)?;
            self.max_entries_to_archive.write_xdr(w)?;
            self.live_soroban_state_size_window_sample_size
                .write_xdr(w)?;
            self.live_soroban_state_size_window_sample_period
                .write_xdr(w)?;
            self.eviction_scan_size.write_xdr(w)?;
            self.starting_eviction_scan_level.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`StateArchivalSettings`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyStateArchivalSettings(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyStateArchivalSettings {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyStateArchivalSettings {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.max_entry_ttl().cmp(&other.max_entry_ttl()))
            .then_with(|| self.min_temporary_ttl().cmp(&other.min_temporary_ttl()))
            .then_with(|| self.min_persistent_ttl().cmp(&other.min_persistent_ttl()))
            .then_with(|| {
                self.persistent_rent_rate_denominator()
                    .cmp(&other.persistent_rent_rate_denominator())
            })
            .then_with(|| {
                self.temp_rent_rate_denominator()
                    .cmp(&other.temp_rent_rate_denominator())
            })
            .then_with(|| {
                self.max_entries_to_archive()
                    .cmp(&other.max_entries_to_archive())
            })
            .then_with(|| {
                self.live_soroban_state_size_window_sample_size()
                    .cmp(&other.live_soroban_state_size_window_sample_size())
            })
            .then_with(|| {
                self.live_soroban_state_size_window_sample_period()
                    .cmp(&other.live_soroban_state_size_window_sample_period())
            })
            .then_with(|| self.eviction_scan_size().cmp(&other.eviction_scan_size()))
            .then_with(|| {
                self.starting_eviction_scan_level()
                    .cmp(&other.starting_eviction_scan_level())
            })
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyStateArchivalSettings {
    const FIXED_XDR_SIZE: Option<u32> = Some(48);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(48).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        48
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyStateArchivalSettings {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyStateArchivalSettings {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyStateArchivalSettings {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyStateArchivalSettings {
    /// Access field `max_entry_ttl`.
    #[must_use]
    pub fn max_entry_ttl(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `min_temporary_ttl`.
    #[must_use]
    pub fn min_temporary_ttl(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 4)
    }
    /// Access field `min_persistent_ttl`.
    #[must_use]
    pub fn min_persistent_ttl(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 8)
    }
    /// Access field `persistent_rent_rate_denominator`.
    #[must_use]
    pub fn persistent_rent_rate_denominator(&self) -> i64 {
        <i64 as LazyXdr>::from_xdr_at(&self.0, 12)
    }
    /// Access field `temp_rent_rate_denominator`.
    #[must_use]
    pub fn temp_rent_rate_denominator(&self) -> i64 {
        <i64 as LazyXdr>::from_xdr_at(&self.0, 20)
    }
    /// Access field `max_entries_to_archive`.
    #[must_use]
    pub fn max_entries_to_archive(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 28)
    }
    /// Access field `live_soroban_state_size_window_sample_size`.
    #[must_use]
    pub fn live_soroban_state_size_window_sample_size(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 32)
    }
    /// Access field `live_soroban_state_size_window_sample_period`.
    #[must_use]
    pub fn live_soroban_state_size_window_sample_period(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 36)
    }
    /// Access field `eviction_scan_size`.
    #[must_use]
    pub fn eviction_scan_size(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 40)
    }
    /// Access field `starting_eviction_scan_level`.
    #[must_use]
    pub fn starting_eviction_scan_level(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 44)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&StateArchivalSettings> for LazyStateArchivalSettings {
    type Error = Error;
    fn try_from(val: &StateArchivalSettings) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyStateArchivalSettings> for StateArchivalSettings {
    type Error = Error;
    fn try_from(lazy: &LazyStateArchivalSettings) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
