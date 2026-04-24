#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ConfigSettingScpTiming is an XDR Struct defined as:
///
/// ```text
/// struct ConfigSettingSCPTiming {
///     uint32 ledgerTargetCloseTimeMilliseconds;
///     uint32 nominationTimeoutInitialMilliseconds;
///     uint32 nominationTimeoutIncrementMilliseconds;
///     uint32 ballotTimeoutInitialMilliseconds;
///     uint32 ballotTimeoutIncrementMilliseconds;
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
pub struct ConfigSettingScpTiming {
    pub ledger_target_close_time_milliseconds: u32,
    pub nomination_timeout_initial_milliseconds: u32,
    pub nomination_timeout_increment_milliseconds: u32,
    pub ballot_timeout_initial_milliseconds: u32,
    pub ballot_timeout_increment_milliseconds: u32,
}

impl ReadXdr for ConfigSettingScpTiming {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ledger_target_close_time_milliseconds: u32::read_xdr(r)?,
                nomination_timeout_initial_milliseconds: u32::read_xdr(r)?,
                nomination_timeout_increment_milliseconds: u32::read_xdr(r)?,
                ballot_timeout_initial_milliseconds: u32::read_xdr(r)?,
                ballot_timeout_increment_milliseconds: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ConfigSettingScpTiming {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_target_close_time_milliseconds.write_xdr(w)?;
            self.nomination_timeout_initial_milliseconds.write_xdr(w)?;
            self.nomination_timeout_increment_milliseconds
                .write_xdr(w)?;
            self.ballot_timeout_initial_milliseconds.write_xdr(w)?;
            self.ballot_timeout_increment_milliseconds.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ConfigSettingScpTiming`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyConfigSettingScpTiming(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyConfigSettingScpTiming {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyConfigSettingScpTiming {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| {
                self.ledger_target_close_time_milliseconds()
                    .cmp(&other.ledger_target_close_time_milliseconds())
            })
            .then_with(|| {
                self.nomination_timeout_initial_milliseconds()
                    .cmp(&other.nomination_timeout_initial_milliseconds())
            })
            .then_with(|| {
                self.nomination_timeout_increment_milliseconds()
                    .cmp(&other.nomination_timeout_increment_milliseconds())
            })
            .then_with(|| {
                self.ballot_timeout_initial_milliseconds()
                    .cmp(&other.ballot_timeout_initial_milliseconds())
            })
            .then_with(|| {
                self.ballot_timeout_increment_milliseconds()
                    .cmp(&other.ballot_timeout_increment_milliseconds())
            })
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyConfigSettingScpTiming {
    const FIXED_XDR_SIZE: Option<u32> = Some(20);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(20).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        20
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
impl From<LazyHandle> for LazyConfigSettingScpTiming {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyConfigSettingScpTiming {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyConfigSettingScpTiming {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyConfigSettingScpTiming {
    /// Access field `ledger_target_close_time_milliseconds`.
    #[must_use]
    pub fn ledger_target_close_time_milliseconds(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `nomination_timeout_initial_milliseconds`.
    #[must_use]
    pub fn nomination_timeout_initial_milliseconds(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 4)
    }
    /// Access field `nomination_timeout_increment_milliseconds`.
    #[must_use]
    pub fn nomination_timeout_increment_milliseconds(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 8)
    }
    /// Access field `ballot_timeout_initial_milliseconds`.
    #[must_use]
    pub fn ballot_timeout_initial_milliseconds(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 12)
    }
    /// Access field `ballot_timeout_increment_milliseconds`.
    #[must_use]
    pub fn ballot_timeout_increment_milliseconds(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 16)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ConfigSettingScpTiming> for LazyConfigSettingScpTiming {
    type Error = Error;
    fn try_from(val: &ConfigSettingScpTiming) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyConfigSettingScpTiming> for ConfigSettingScpTiming {
    type Error = Error;
    fn try_from(lazy: &LazyConfigSettingScpTiming) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
