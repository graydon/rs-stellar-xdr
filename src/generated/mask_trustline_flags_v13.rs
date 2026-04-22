#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// MaskTrustlineFlagsV13 is an XDR Const defined as:
///
/// ```text
/// const MASK_TRUSTLINE_FLAGS_V13 = 3;
/// ```
///
pub const MASK_TRUSTLINE_FLAGS_V13: u64 = 3;
