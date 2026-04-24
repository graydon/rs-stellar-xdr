#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// LiquidityPoolEntryConstantProduct is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///         {
///             LiquidityPoolConstantProductParameters params;
///
///             int64 reserveA;        // amount of A in the pool
///             int64 reserveB;        // amount of B in the pool
///             int64 totalPoolShares; // total number of pool shares issued
///             int64 poolSharesTrustLineCount; // number of trust lines for the
///                                             // associated pool shares
///         }
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
pub struct LiquidityPoolEntryConstantProduct {
    pub params: LiquidityPoolConstantProductParameters,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub reserve_a: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub reserve_b: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub total_pool_shares: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub pool_shares_trust_line_count: i64,
}

impl ReadXdr for LiquidityPoolEntryConstantProduct {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                params: LiquidityPoolConstantProductParameters::read_xdr(r)?,
                reserve_a: i64::read_xdr(r)?,
                reserve_b: i64::read_xdr(r)?,
                total_pool_shares: i64::read_xdr(r)?,
                pool_shares_trust_line_count: i64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LiquidityPoolEntryConstantProduct {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.params.write_xdr(w)?;
            self.reserve_a.write_xdr(w)?;
            self.reserve_b.write_xdr(w)?;
            self.total_pool_shares.write_xdr(w)?;
            self.pool_shares_trust_line_count.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`LiquidityPoolEntryConstantProduct`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyLiquidityPoolEntryConstantProduct(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyLiquidityPoolEntryConstantProduct {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyLiquidityPoolEntryConstantProduct {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.params().cmp(&other.params()))
            .then_with(|| self.reserve_a().cmp(&other.reserve_a()))
            .then_with(|| self.reserve_b().cmp(&other.reserve_b()))
            .then_with(|| self.total_pool_shares().cmp(&other.total_pool_shares()))
            .then_with(|| {
                self.pool_shares_trust_line_count()
                    .cmp(&other.pool_shares_trust_line_count())
            })
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyLiquidityPoolEntryConstantProduct {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazyLiquidityPoolConstantProductParameters as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(32).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos +=
            <LazyLiquidityPoolConstantProductParameters as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 32;
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
impl From<LazyHandle> for LazyLiquidityPoolEntryConstantProduct {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyLiquidityPoolEntryConstantProduct {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyLiquidityPoolEntryConstantProduct {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyLiquidityPoolEntryConstantProduct {
    /// Access field `params`.
    #[must_use]
    pub fn params(&self) -> LazyLiquidityPoolConstantProductParameters {
        <LazyLiquidityPoolConstantProductParameters as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `reserve_a`.
    #[must_use]
    pub fn reserve_a(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos +=
            <LazyLiquidityPoolConstantProductParameters as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `reserve_b`.
    #[must_use]
    pub fn reserve_b(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos +=
            <LazyLiquidityPoolConstantProductParameters as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `total_pool_shares`.
    #[must_use]
    pub fn total_pool_shares(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos +=
            <LazyLiquidityPoolConstantProductParameters as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 16;
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `pool_shares_trust_line_count`.
    #[must_use]
    pub fn pool_shares_trust_line_count(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos +=
            <LazyLiquidityPoolConstantProductParameters as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 24;
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LiquidityPoolEntryConstantProduct> for LazyLiquidityPoolEntryConstantProduct {
    type Error = Error;
    fn try_from(val: &LiquidityPoolEntryConstantProduct) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyLiquidityPoolEntryConstantProduct> for LiquidityPoolEntryConstantProduct {
    type Error = Error;
    fn try_from(lazy: &LazyLiquidityPoolEntryConstantProduct) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
