#![doc = include_str!("../README.md")]
#![no_std]
#![feature(asm, never_type)]
#![warn(missing_docs)]
#![cfg(any(
    target_arch = "riscv32",
    target_arch = "riscv64",
    target_arch = "riscv128",
    doc,
))]

use core::convert::TryFrom;

pub mod base;
pub mod srst;
pub mod time;
pub mod legacy;

/// A raw value returned from an SBI call.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Ret {
    /// The error returned.
    pub error: usize,
    /// The value returned.
    pub value: usize,
}

impl From<Ret> for Result<usize, StandardError> {
    fn from(ret: Ret) -> Self {
        use core::convert::TryInto;
        if let Ok(error) = ret.error.try_into() {
            Err(error)
        } else {
            Ok(ret.value)
        }
    }
}

/// A standard error returned from an SBI call.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[non_exhaustive]
pub enum StandardError {
    Failed,
    NotSupported,
    InvalidParam,
    Denied,
    InvalidAddr,
    AlreadyAvailable,
    AlreadyStarted,
    AlreadyStopped,
    Unknown,
}

impl TryFrom<usize> for StandardError {
    type Error = ();
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value as isize {
            0 => Err(()),
            -1 => Ok(StandardError::Failed),
            -2 => Ok(StandardError::NotSupported),
            -3 => Ok(StandardError::InvalidParam),
            -4 => Ok(StandardError::Denied),
            -5 => Ok(StandardError::InvalidAddr),
            -6 => Ok(StandardError::AlreadyAvailable),
            -7 => Ok(StandardError::AlreadyStarted),
            -8 => Ok(StandardError::AlreadyStopped),
            _ => Ok(StandardError::Unknown),
        }
    }
}

const NUM_ARGS: usize = 6;

/// The `ecall` instruction is used to make SBI calls. This function is exposed
/// to allow access to experimental, vendor-specific, and firmware-specific SBI
/// extensions. For core extensions, you should prefer using the functions in
/// the individual modules of this crate.
///
/// # Safety
/// The behavior of this function is undefined if it is not used to make 
/// SBI-conforming calls.
#[inline(always)]
pub fn ecall(eid: u32, fid: u32, args: [usize; NUM_ARGS]) -> Ret {
    let mut error;
    let mut value;
    unsafe {
        asm!(
            "ecall",
            in("a0") args[0],
            in("a1") args[1],
            in("a2") args[2],
            in("a3") args[3],
            in("a4") args[4],
            in("a5") args[5],
            in("a6") fid,
            in("a7") eid,
            lateout("a0") error,
            lateout("a1") value,
        );
    }
    Ret { error, value }
}
