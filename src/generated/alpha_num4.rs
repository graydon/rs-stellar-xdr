#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// AlphaNum4 is an XDR Struct defined as:
///
/// ```text
/// struct AlphaNum4
/// {
///     AssetCode4 assetCode;
///     AccountID issuer;
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
pub struct AlphaNum4 {
    pub asset_code: AssetCode4,
    pub issuer: AccountId,
}

impl ReadXdr for AlphaNum4 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                asset_code: AssetCode4::read_xdr(r)?,
                issuer: AccountId::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for AlphaNum4 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.asset_code.write_xdr(w)?;
            self.issuer.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`AlphaNum4`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyAlphaNum4(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyAlphaNum4 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyAlphaNum4 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.asset_code().cmp(&other.asset_code()))
            .then_with(|| self.issuer().cmp(&other.issuer()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyAlphaNum4 {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(4).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyAssetCode4 as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        {
            let field_len = <LazyAccountId as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += 4;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyAlphaNum4 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyAlphaNum4 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyAlphaNum4 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyAlphaNum4 {
    /// Access field `asset_code`.
    #[must_use]
    pub fn asset_code(&self) -> LazyAssetCode4 {
        <LazyAssetCode4 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `issuer`.
    #[must_use]
    pub fn issuer(&self) -> LazyAccountId {
        <LazyAccountId as LazyXdr>::from_xdr_at(&self.0, 4)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&AlphaNum4> for LazyAlphaNum4 {
    type Error = Error;
    fn try_from(val: &AlphaNum4) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyAlphaNum4> for AlphaNum4 {
    type Error = Error;
    fn try_from(lazy: &LazyAlphaNum4) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
