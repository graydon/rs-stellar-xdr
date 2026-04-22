#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// Uint32 is an XDR Typedef defined as:
///
/// ```text
/// typedef unsigned int uint32;
/// ```
///
pub type Uint32 = u32;

#[cfg(feature = "alloc")]
/// Lazy alias — scalars need no wrapper.
pub type LazyUint32 = u32;
