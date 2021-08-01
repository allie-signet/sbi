//! The timer extension permits the supervisor to program the clock
//! for the next timer event.

use crate::StandardError;

const EID: u32 = 0x54494d45;

/// Program the clock for the next timer event and clears the timer interrupt
/// pending bit. The provided value is an absolute time.
pub fn set_timer(stime_value: u64) -> Result<(), StandardError> {
    use {
        crate::ecall,
        {bit_field::BitField, core::convert::TryInto},
    };
    const FID: u32 = 0x0;
    let ret = if let Ok(stime_value) = stime_value.try_into() {
        ecall(EID, FID, [stime_value, 0, 0, 0, 0, 0])
    } else {
        ecall(
            EID,
            FID,
            [
                stime_value.get_bits(0..32) as usize,
                stime_value.get_bits(32..64) as usize,
                0,
                0,
                0,
                0,
            ],
        )
    };
    let res: Result<_, _> = ret.into();
    res.map(|_| ())
}
