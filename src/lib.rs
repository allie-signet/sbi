//! A Rust wrapper around the RISC-V Supervisor Binary Interface (SBI).
//!
//! This crate aims to provide an idiomatic Rust interface to all the core
//! extensions of [SBI v0.2.0+](https://github.com/riscv/riscv-sbi-doc).
#![no_std]
#![feature(asm)]
#![deny(missing_docs)]

use ::core::num::NonZeroIsize;

/// A unique identifier for an extension.
pub type ExtensionId = i32;

/// A unique identifier for a function within an extension.
pub type FunctionId = i32;

/// A raw value for an argument to an SBI call.
pub type ArgValue = isize;

/// A raw value for a return from an SBI call.
pub type RetValue = isize;

/// A raw error for a return from an SBI call.
pub type RetError = NonZeroIsize;

/// The `ecall` instruction is used to make SBI calls. This function is exposed
/// to allow access to experimental, vendor-specific, and firmware-specific SBI
/// extensions. For core extensions, you should prefer using the individual
/// modules.
pub fn ecall(
    eid: ExtensionId,
    fid: FunctionId,
    args: (ArgValue, ArgValue, ArgValue),
) -> Result<RetValue, (RetValue, RetError)> {
    #[cfg(any(
        target_arch = "riscv32",
        target_arch = "riscv64",
        target_arch = "riscv128",
    ))]
    {
        let error = 0isize;
        let value = 0isize;
        unsafe {
            asm!(
                "ecall",
                in("a0") args.0,
                in("a1") args.1,
                in("a2") args.2,
                in("a6") fid,
                in("a7") eid,
                lateout("a0") error,
                lateout("a1") value,
            );
        }
        if let Some(error) = NonZeroIsize::new(error) {
            Err((value, error))
        } else {
            Ok(value)
        }
    }
    #[cfg(not(any(
        target_arch = "riscv32",
        target_arch = "riscv64",
        target_arch = "riscv128"
    )))]
    {
        drop((eid, fid, args));
        unimplemented!("SBI is only supported on RISC-V");
    }
}

pub mod base;
pub mod hsm;
pub mod ipi;
pub mod reset;
pub mod rfence;
pub mod timer;
