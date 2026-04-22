#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ScsymbolLimit is an XDR Const defined as:
///
/// ```text
/// const SCSYMBOL_LIMIT = 32;
/// ```
///
pub const SCSYMBOL_LIMIT: u64 = 32;
