//! The rfence extension permits the supervisor to force fences on
//! remote harts.

use crate::ExtensionId;

/// Extension identifier for the rfence extension.
pub const EID: ExtensionId = 0x52464e43;
