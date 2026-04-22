#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ContractCostCountLimit is an XDR Const defined as:
///
/// ```text
/// const CONTRACT_COST_COUNT_LIMIT = 1024;
/// ```
///
pub const CONTRACT_COST_COUNT_LIMIT: u64 = 1024;
