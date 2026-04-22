#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// Int32 is an XDR Typedef defined as:
///
/// ```text
/// typedef int int32;
/// ```
///
pub type Int32 = i32;

#[cfg(feature = "alloc")]
/// Lazy alias — scalars need no wrapper.
pub type LazyInt32 = i32;
