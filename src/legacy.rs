//! The legacy extensions are deprecated in favor of the other extensions.
//! However, the legacy console extensions have no replacement, and so are 
//! provided here.

/// Write a byte to the debug console, if present.
pub fn console_put(b: u8) {
    use crate::ecall;
    
    ecall(0x1, 0x0, [b as usize, 0, 0, 0, 0, 0]);
}

/// Read a character to the debug console, if present.
pub fn console_get() -> Option<u8> {
    use {::core::convert::TryInto, crate::ecall};
    
    let ret = ecall(0x2, 0x0, Default::default());
    
    if let Ok(b) = ret.value.try_into() {
        Some(b)
    } else {
        None
    }
}
