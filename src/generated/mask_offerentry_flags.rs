#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// MaskOfferentryFlags is an XDR Const defined as:
///
/// ```text
/// const MASK_OFFERENTRY_FLAGS = 1;
/// ```
///
pub const MASK_OFFERENTRY_FLAGS: u64 = 1;
