#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// AuthMsgFlagFlowControlBytesRequested is an XDR Const defined as:
///
/// ```text
/// const AUTH_MSG_FLAG_FLOW_CONTROL_BYTES_REQUESTED = 200;
/// ```
///
pub const AUTH_MSG_FLAG_FLOW_CONTROL_BYTES_REQUESTED: u64 = 200;
