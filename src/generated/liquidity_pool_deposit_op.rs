#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// LiquidityPoolDepositOp is an XDR Struct defined as:
///
/// ```text
/// struct LiquidityPoolDepositOp
/// {
///     PoolID liquidityPoolID;
///     int64 maxAmountA; // maximum amount of first asset to deposit
///     int64 maxAmountB; // maximum amount of second asset to deposit
///     Price minPrice;   // minimum depositA/depositB
///     Price maxPrice;   // maximum depositA/depositB
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
pub struct LiquidityPoolDepositOp {
    pub liquidity_pool_id: PoolId,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub max_amount_a: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub max_amount_b: i64,
    pub min_price: Price,
    pub max_price: Price,
}

impl ReadXdr for LiquidityPoolDepositOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                liquidity_pool_id: PoolId::read_xdr(r)?,
                max_amount_a: i64::read_xdr(r)?,
                max_amount_b: i64::read_xdr(r)?,
                min_price: Price::read_xdr(r)?,
                max_price: Price::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LiquidityPoolDepositOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.liquidity_pool_id.write_xdr(w)?;
            self.max_amount_a.write_xdr(w)?;
            self.max_amount_b.write_xdr(w)?;
            self.min_price.write_xdr(w)?;
            self.max_price.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`LiquidityPoolDepositOp`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyLiquidityPoolDepositOp(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyLiquidityPoolDepositOp {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyLiquidityPoolDepositOp {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.liquidity_pool_id().cmp(&other.liquidity_pool_id()))
            .then_with(|| self.max_amount_a().cmp(&other.max_amount_a()))
            .then_with(|| self.max_amount_b().cmp(&other.max_amount_b()))
            .then_with(|| self.min_price().cmp(&other.min_price()))
            .then_with(|| self.max_price().cmp(&other.max_price()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyLiquidityPoolDepositOp {
    const FIXED_XDR_SIZE: Option<u32> = Some(64);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(64).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyPoolId as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        <LazyPrice as LazyXdr>::xdr_validate(&buf[(pos + 48) as usize..], depth)?;
        <LazyPrice as LazyXdr>::xdr_validate(&buf[(pos + 56) as usize..], depth)?;
        pos = next_pos;
        Ok(pos)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        64
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyLiquidityPoolDepositOp {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyLiquidityPoolDepositOp {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyLiquidityPoolDepositOp {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyLiquidityPoolDepositOp {
    /// Access field `liquidity_pool_id`.
    #[must_use]
    pub fn liquidity_pool_id(&self) -> LazyPoolId {
        <LazyPoolId as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `max_amount_a`.
    #[must_use]
    pub fn max_amount_a(&self) -> i64 {
        <i64 as LazyXdr>::from_xdr_at(&self.0, 32)
    }
    /// Access field `max_amount_b`.
    #[must_use]
    pub fn max_amount_b(&self) -> i64 {
        <i64 as LazyXdr>::from_xdr_at(&self.0, 40)
    }
    /// Access field `min_price`.
    #[must_use]
    pub fn min_price(&self) -> LazyPrice {
        <LazyPrice as LazyXdr>::from_xdr_at(&self.0, 48)
    }
    /// Access field `max_price`.
    #[must_use]
    pub fn max_price(&self) -> LazyPrice {
        <LazyPrice as LazyXdr>::from_xdr_at(&self.0, 56)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LiquidityPoolDepositOp> for LazyLiquidityPoolDepositOp {
    type Error = Error;
    fn try_from(val: &LiquidityPoolDepositOp) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyLiquidityPoolDepositOp> for LiquidityPoolDepositOp {
    type Error = Error;
    fn try_from(lazy: &LazyLiquidityPoolDepositOp) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
