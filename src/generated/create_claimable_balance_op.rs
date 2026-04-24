#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// CreateClaimableBalanceOp is an XDR Struct defined as:
///
/// ```text
/// struct CreateClaimableBalanceOp
/// {
///     Asset asset;
///     int64 amount;
///     Claimant claimants<10>;
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
pub struct CreateClaimableBalanceOp {
    pub asset: Asset,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub amount: i64,
    pub claimants: VecM<Claimant, 10>,
}

impl ReadXdr for CreateClaimableBalanceOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                asset: Asset::read_xdr(r)?,
                amount: i64::read_xdr(r)?,
                claimants: VecM::<Claimant, 10>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for CreateClaimableBalanceOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.asset.write_xdr(w)?;
            self.amount.write_xdr(w)?;
            self.claimants.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`CreateClaimableBalanceOp`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyCreateClaimableBalanceOp(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyCreateClaimableBalanceOp {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyCreateClaimableBalanceOp {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.asset().cmp(&other.asset()))
            .then_with(|| self.amount().cmp(&other.amount()))
            .then_with(|| self.claimants().cmp(&other.claimants()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyCreateClaimableBalanceOp {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazyAsset as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(8).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        {
            let field_len =
                <LazyVecM<LazyClaimant, 10> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        pos += <LazyVecM<LazyClaimant, 10> as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyCreateClaimableBalanceOp {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyCreateClaimableBalanceOp {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyCreateClaimableBalanceOp {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyCreateClaimableBalanceOp {
    /// Access field `asset`.
    #[must_use]
    pub fn asset(&self) -> LazyAsset {
        <LazyAsset as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `amount`.
    #[must_use]
    pub fn amount(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `claimants`.
    #[must_use]
    pub fn claimants(&self) -> LazyVecM<LazyClaimant, 10> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        <LazyVecM<LazyClaimant, 10> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&CreateClaimableBalanceOp> for LazyCreateClaimableBalanceOp {
    type Error = Error;
    fn try_from(val: &CreateClaimableBalanceOp) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyCreateClaimableBalanceOp> for CreateClaimableBalanceOp {
    type Error = Error;
    fn try_from(lazy: &LazyCreateClaimableBalanceOp) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
