#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// MaxSigners is an XDR Const defined as:
///
/// ```text
/// const MAX_SIGNERS = 20;
/// ```
///
pub const MAX_SIGNERS: u64 = 20;
