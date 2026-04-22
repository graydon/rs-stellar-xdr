#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// MaskClaimableBalanceFlags is an XDR Const defined as:
///
/// ```text
/// const MASK_CLAIMABLE_BALANCE_FLAGS = 0x1;
/// ```
///
pub const MASK_CLAIMABLE_BALANCE_FLAGS: u64 = 0x1;
