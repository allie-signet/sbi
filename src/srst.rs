//! The system reset extension permits the supervisor to manage
//! system power state.

use crate::StandardError;

const EID: u32 = 0x53525354;

/// A type of system reset.
pub enum Type {
    Shutdown,
    ColdReboot,
    WarmReboot,
}

impl From<Type> for usize {
    fn from(t: Type) -> Self {
        match t {
            Type::Shutdown => 0x00000000,
            Type::ColdReboot => 0x00000001,
            Type::WarmReboot => 0x00000002,
        }
    }
}

/// A reason for a system reset.
pub enum Reason {
    NoReason,
    SystemFailure,
}

impl From<Reason> for usize {
    fn from(r: Reason) -> Self {
        match r {
            Reason::NoReason => 0x00000000,
            Reason::SystemFailure => 0x00000001,
        }
    }
}

/// Synchronously reset the system with a given type and reason.
pub fn system_reset(t: Type, r: Reason) -> Result<!, StandardError> {
    use crate::ecall;
    let ret = ecall(EID, 0x0, [t.into(), r.into(), 0, 0, 0, 0]);
    let res: Result<_, _> = ret.into();
    res.map(|_| unreachable!())
}
