#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// Uint64 is an XDR Typedef defined as:
///
/// ```text
/// typedef unsigned hyper uint64;
/// ```
///
pub type Uint64 = u64;

#[cfg(feature = "alloc")]
/// Lazy alias — scalars need no wrapper.
pub type LazyUint64 = u64;
