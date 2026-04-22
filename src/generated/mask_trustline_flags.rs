#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// MaskTrustlineFlags is an XDR Const defined as:
///
/// ```text
/// const MASK_TRUSTLINE_FLAGS = 1;
/// ```
///
pub const MASK_TRUSTLINE_FLAGS: u64 = 1;
