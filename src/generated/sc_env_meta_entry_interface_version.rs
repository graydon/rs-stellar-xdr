#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ScEnvMetaEntryInterfaceVersion is an XDR NestedStruct defined as:
///
/// ```text
/// struct {
///         uint32 protocol;
///         uint32 preRelease;
///     }
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
pub struct ScEnvMetaEntryInterfaceVersion {
    pub protocol: u32,
    pub pre_release: u32,
}

impl ReadXdr for ScEnvMetaEntryInterfaceVersion {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                protocol: u32::read_xdr(r)?,
                pre_release: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScEnvMetaEntryInterfaceVersion {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.protocol.write_xdr(w)?;
            self.pre_release.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ScEnvMetaEntryInterfaceVersion`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyScEnvMetaEntryInterfaceVersion(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyScEnvMetaEntryInterfaceVersion {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyScEnvMetaEntryInterfaceVersion {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.protocol().cmp(&other.protocol()))
            .then_with(|| self.pre_release().cmp(&other.pre_release()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyScEnvMetaEntryInterfaceVersion {
    const FIXED_XDR_SIZE: Option<u32> = Some(8);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(8).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        8
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyScEnvMetaEntryInterfaceVersion {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyScEnvMetaEntryInterfaceVersion {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyScEnvMetaEntryInterfaceVersion {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyScEnvMetaEntryInterfaceVersion {
    /// Access field `protocol`.
    #[must_use]
    pub fn protocol(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `pre_release`.
    #[must_use]
    pub fn pre_release(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 4)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ScEnvMetaEntryInterfaceVersion> for LazyScEnvMetaEntryInterfaceVersion {
    type Error = Error;
    fn try_from(val: &ScEnvMetaEntryInterfaceVersion) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyScEnvMetaEntryInterfaceVersion> for ScEnvMetaEntryInterfaceVersion {
    type Error = Error;
    fn try_from(lazy: &LazyScEnvMetaEntryInterfaceVersion) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
