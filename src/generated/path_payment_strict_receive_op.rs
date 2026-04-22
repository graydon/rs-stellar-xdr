#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// PathPaymentStrictReceiveOp is an XDR Struct defined as:
///
/// ```text
/// struct PathPaymentStrictReceiveOp
/// {
///     Asset sendAsset; // asset we pay with
///     int64 sendMax;   // the maximum amount of sendAsset to
///                      // send (excluding fees).
///                      // The operation will fail if can't be met
///
///     MuxedAccount destination; // recipient of the payment
///     Asset destAsset;          // what they end up with
///     int64 destAmount;         // amount they end up with
///
///     Asset path<5>; // additional hops it must go through to get there
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
pub struct PathPaymentStrictReceiveOp {
    pub send_asset: Asset,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub send_max: i64,
    pub destination: MuxedAccount,
    pub dest_asset: Asset,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub dest_amount: i64,
    pub path: VecM<Asset, 5>,
}

impl ReadXdr for PathPaymentStrictReceiveOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                send_asset: Asset::read_xdr(r)?,
                send_max: i64::read_xdr(r)?,
                destination: MuxedAccount::read_xdr(r)?,
                dest_asset: Asset::read_xdr(r)?,
                dest_amount: i64::read_xdr(r)?,
                path: VecM::<Asset, 5>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for PathPaymentStrictReceiveOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.send_asset.write_xdr(w)?;
            self.send_max.write_xdr(w)?;
            self.destination.write_xdr(w)?;
            self.dest_asset.write_xdr(w)?;
            self.dest_amount.write_xdr(w)?;
            self.path.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`PathPaymentStrictReceiveOp`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyPathPaymentStrictReceiveOp(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyPathPaymentStrictReceiveOp {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyPathPaymentStrictReceiveOp {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.send_asset().cmp(&other.send_asset()))
            .then_with(|| self.send_max().cmp(&other.send_max()))
            .then_with(|| self.destination().cmp(&other.destination()))
            .then_with(|| self.dest_asset().cmp(&other.dest_asset()))
            .then_with(|| self.dest_amount().cmp(&other.dest_amount()))
            .then_with(|| self.path().cmp(&other.path()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyPathPaymentStrictReceiveOp {
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
                <LazyMuxedAccount as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
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
        {
            let field_len =
                <LazyVecM<LazyAsset, 5> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        pos += <LazyMuxedAccount as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        pos += <LazyVecM<LazyAsset, 5> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyPathPaymentStrictReceiveOp {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyPathPaymentStrictReceiveOp {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyPathPaymentStrictReceiveOp {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyPathPaymentStrictReceiveOp {
    /// Access field `send_asset`.
    #[must_use]
    pub fn send_asset(&self) -> LazyAsset {
        <LazyAsset as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `send_max`.
    #[must_use]
    pub fn send_max(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `destination`.
    #[must_use]
    pub fn destination(&self) -> LazyMuxedAccount {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        <LazyMuxedAccount as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `dest_asset`.
    #[must_use]
    pub fn dest_asset(&self) -> LazyAsset {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        pos += <LazyMuxedAccount as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyAsset as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `dest_amount`.
    #[must_use]
    pub fn dest_amount(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        pos += <LazyMuxedAccount as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `path`.
    #[must_use]
    pub fn path(&self) -> LazyVecM<LazyAsset, 5> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        pos += <LazyMuxedAccount as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyAsset as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        <LazyVecM<LazyAsset, 5> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&PathPaymentStrictReceiveOp> for LazyPathPaymentStrictReceiveOp {
    type Error = Error;
    fn try_from(val: &PathPaymentStrictReceiveOp) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyPathPaymentStrictReceiveOp> for PathPaymentStrictReceiveOp {
    type Error = Error;
    fn try_from(lazy: &LazyPathPaymentStrictReceiveOp) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
