#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// LiquidityPoolWithdrawOp is an XDR Struct defined as:
///
/// ```text
/// struct LiquidityPoolWithdrawOp
/// {
///     PoolID liquidityPoolID;
///     int64 amount;     // amount of pool shares to withdraw
///     int64 minAmountA; // minimum amount of first asset to withdraw
///     int64 minAmountB; // minimum amount of second asset to withdraw
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
pub struct LiquidityPoolWithdrawOp {
    pub liquidity_pool_id: PoolId,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub amount: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub min_amount_a: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub min_amount_b: i64,
}

impl ReadXdr for LiquidityPoolWithdrawOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                liquidity_pool_id: PoolId::read_xdr(r)?,
                amount: i64::read_xdr(r)?,
                min_amount_a: i64::read_xdr(r)?,
                min_amount_b: i64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LiquidityPoolWithdrawOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.liquidity_pool_id.write_xdr(w)?;
            self.amount.write_xdr(w)?;
            self.min_amount_a.write_xdr(w)?;
            self.min_amount_b.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`LiquidityPoolWithdrawOp`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyLiquidityPoolWithdrawOp(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyLiquidityPoolWithdrawOp {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyLiquidityPoolWithdrawOp {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.liquidity_pool_id().cmp(&other.liquidity_pool_id()))
            .then_with(|| self.amount().cmp(&other.amount()))
            .then_with(|| self.min_amount_a().cmp(&other.min_amount_a()))
            .then_with(|| self.min_amount_b().cmp(&other.min_amount_b()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyLiquidityPoolWithdrawOp {
    const FIXED_XDR_SIZE: Option<u32> = Some(56);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(56).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyPoolId as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        Ok(pos)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        56
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyLiquidityPoolWithdrawOp {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyLiquidityPoolWithdrawOp {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyLiquidityPoolWithdrawOp {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyLiquidityPoolWithdrawOp {
    /// Access field `liquidity_pool_id`.
    #[must_use]
    pub fn liquidity_pool_id(&self) -> LazyPoolId {
        <LazyPoolId as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `amount`.
    #[must_use]
    pub fn amount(&self) -> i64 {
        <i64 as LazyXdr>::from_xdr_at(&self.0, 32)
    }
    /// Access field `min_amount_a`.
    #[must_use]
    pub fn min_amount_a(&self) -> i64 {
        <i64 as LazyXdr>::from_xdr_at(&self.0, 40)
    }
    /// Access field `min_amount_b`.
    #[must_use]
    pub fn min_amount_b(&self) -> i64 {
        <i64 as LazyXdr>::from_xdr_at(&self.0, 48)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LiquidityPoolWithdrawOp> for LazyLiquidityPoolWithdrawOp {
    type Error = Error;
    fn try_from(val: &LiquidityPoolWithdrawOp) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyLiquidityPoolWithdrawOp> for LiquidityPoolWithdrawOp {
    type Error = Error;
    fn try_from(lazy: &LazyLiquidityPoolWithdrawOp) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
