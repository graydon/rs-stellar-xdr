#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// TxAdvertVectorMaxSize is an XDR Const defined as:
///
/// ```text
/// const TX_ADVERT_VECTOR_MAX_SIZE = 1000;
/// ```
///
pub const TX_ADVERT_VECTOR_MAX_SIZE: u64 = 1000;
