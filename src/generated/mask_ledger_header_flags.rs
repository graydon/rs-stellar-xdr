#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// MaskLedgerHeaderFlags is an XDR Const defined as:
///
/// ```text
/// const MASK_LEDGER_HEADER_FLAGS = 0x7;
/// ```
///
pub const MASK_LEDGER_HEADER_FLAGS: u64 = 0x7;
