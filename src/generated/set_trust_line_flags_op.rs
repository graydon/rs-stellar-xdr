#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SetTrustLineFlagsOp is an XDR Struct defined as:
///
/// ```text
/// struct SetTrustLineFlagsOp
/// {
///     AccountID trustor;
///     Asset asset;
///
///     uint32 clearFlags; // which flags to clear
///     uint32 setFlags;   // which flags to set
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
pub struct SetTrustLineFlagsOp {
    pub trustor: AccountId,
    pub asset: Asset,
    pub clear_flags: u32,
    pub set_flags: u32,
}

impl ReadXdr for SetTrustLineFlagsOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                trustor: AccountId::read_xdr(r)?,
                asset: Asset::read_xdr(r)?,
                clear_flags: u32::read_xdr(r)?,
                set_flags: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SetTrustLineFlagsOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.trustor.write_xdr(w)?;
            self.asset.write_xdr(w)?;
            self.clear_flags.write_xdr(w)?;
            self.set_flags.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`SetTrustLineFlagsOp`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySetTrustLineFlagsOp(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazySetTrustLineFlagsOp {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazySetTrustLineFlagsOp {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.trustor().cmp(&other.trustor()))
            .then_with(|| self.asset().cmp(&other.asset()))
            .then_with(|| self.clear_flags().cmp(&other.clear_flags()))
            .then_with(|| self.set_flags().cmp(&other.set_flags()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazySetTrustLineFlagsOp {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazyAccountId as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyAsset as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(8).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
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
impl From<LazyHandle> for LazySetTrustLineFlagsOp {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazySetTrustLineFlagsOp {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazySetTrustLineFlagsOp {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazySetTrustLineFlagsOp {
    /// Access field `trustor`.
    #[must_use]
    pub fn trustor(&self) -> LazyAccountId {
        <LazyAccountId as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `asset`.
    #[must_use]
    pub fn asset(&self) -> LazyAsset {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyAsset as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `clear_flags`.
    #[must_use]
    pub fn clear_flags(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `set_flags`.
    #[must_use]
    pub fn set_flags(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&SetTrustLineFlagsOp> for LazySetTrustLineFlagsOp {
    type Error = Error;
    fn try_from(val: &SetTrustLineFlagsOp) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazySetTrustLineFlagsOp> for SetTrustLineFlagsOp {
    type Error = Error;
    fn try_from(lazy: &LazySetTrustLineFlagsOp) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
