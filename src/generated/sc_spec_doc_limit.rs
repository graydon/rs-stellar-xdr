#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ScSpecDocLimit is an XDR Const defined as:
///
/// ```text
/// const SC_SPEC_DOC_LIMIT = 1024;
/// ```
///
pub const SC_SPEC_DOC_LIMIT: u64 = 1024;
