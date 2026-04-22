#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// MaskAccountFlagsV17 is an XDR Const defined as:
///
/// ```text
/// const MASK_ACCOUNT_FLAGS_V17 = 0xF;
/// ```
///
pub const MASK_ACCOUNT_FLAGS_V17: u64 = 0xF;
