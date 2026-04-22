#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// LiquidityPoolFeeV18 is an XDR Const defined as:
///
/// ```text
/// const LIQUIDITY_POOL_FEE_V18 = 30;
/// ```
///
pub const LIQUIDITY_POOL_FEE_V18: u64 = 30;
