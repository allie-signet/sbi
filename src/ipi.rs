//! The IPI extension permits the supervisor to send inter-processor
//! interrupts to a set of harts.

use crate::ExtensionId;

/// Extension identifier for the IPI extension.
pub const EID: ExtensionId = 0x735049;
