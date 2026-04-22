#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// Int64 is an XDR Typedef defined as:
///
/// ```text
/// typedef hyper int64;
/// ```
///
pub type Int64 = i64;

#[cfg(feature = "alloc")]
/// Lazy alias — scalars need no wrapper.
pub type LazyInt64 = i64;
