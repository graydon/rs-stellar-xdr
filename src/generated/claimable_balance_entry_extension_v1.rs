#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ClaimableBalanceEntryExtensionV1 is an XDR Struct defined as:
///
/// ```text
/// struct ClaimableBalanceEntryExtensionV1
/// {
///     union switch (int v)
///     {
///     case 0:
///         void;
///     }
///     ext;
///
///     uint32 flags; // see ClaimableBalanceFlags
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
pub struct ClaimableBalanceEntryExtensionV1 {
    pub ext: ClaimableBalanceEntryExtensionV1Ext,
    pub flags: u32,
}

impl ReadXdr for ClaimableBalanceEntryExtensionV1 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ClaimableBalanceEntryExtensionV1Ext::read_xdr(r)?,
                flags: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ClaimableBalanceEntryExtensionV1 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.flags.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ClaimableBalanceEntryExtensionV1`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyClaimableBalanceEntryExtensionV1(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyClaimableBalanceEntryExtensionV1 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyClaimableBalanceEntryExtensionV1 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.ext().cmp(&other.ext()))
            .then_with(|| self.flags().cmp(&other.flags()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyClaimableBalanceEntryExtensionV1 {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazyClaimableBalanceEntryExtensionV1Ext as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(4).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyClaimableBalanceEntryExtensionV1Ext as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyClaimableBalanceEntryExtensionV1 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyClaimableBalanceEntryExtensionV1 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyClaimableBalanceEntryExtensionV1 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyClaimableBalanceEntryExtensionV1 {
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazyClaimableBalanceEntryExtensionV1Ext {
        <LazyClaimableBalanceEntryExtensionV1Ext as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `flags`.
    #[must_use]
    pub fn flags(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyClaimableBalanceEntryExtensionV1Ext as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ClaimableBalanceEntryExtensionV1> for LazyClaimableBalanceEntryExtensionV1 {
    type Error = Error;
    fn try_from(val: &ClaimableBalanceEntryExtensionV1) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyClaimableBalanceEntryExtensionV1> for ClaimableBalanceEntryExtensionV1 {
    type Error = Error;
    fn try_from(lazy: &LazyClaimableBalanceEntryExtensionV1) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
