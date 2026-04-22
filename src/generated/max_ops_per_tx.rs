#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// MaxOpsPerTx is an XDR Const defined as:
///
/// ```text
/// const MAX_OPS_PER_TX = 100;
/// ```
///
pub const MAX_OPS_PER_TX: u64 = 100;
