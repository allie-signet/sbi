//! The base extension contains all the required functions that any
//! conforming SBI implementation must implement. It permits the
//! supervisor to probe for information about the implementation.

use {crate::ExtensionId, ::core::num::NonZeroIsize};

/// Extension identifier for the base extension.
pub const EID: ExtensionId = 0x10;

/// Denotes the version of the SBI specification that an implementation
/// conforms to.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct SpecVersion {
    /// The major revision of the version.
    pub major: u8,
    /// The minor revision of the version.
    pub minor: u32,
}

/// A unique identifier for an implementation.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ImplId(isize);

/// Denotes the version of an implementation.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ImplVersion(isize);

/// A unique identifier for an machine vendor.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct MachineVendorId(isize);

/// A unique identifier for an machine architecture.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct MachineArchId(isize);

/// A unique identifier for an machine implementation.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct MachineImplId(isize);

/// Denotes availability of an extension and optional extension-specific data.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ExtensionAvailability {
    /// An available extension and optional extension-specific data.
    Available(NonZeroIsize),
    /// An unavailable extension.
    Unavailable,
}

/// Retrieve the version of the SBI specification the implementation
/// conforms to.
pub fn spec_version() -> SpecVersion {
    use {
        crate::{ecall, FunctionId},
        ::bit_field::BitField,
    };
    const FID: FunctionId = 0x0;
    let value =
        ecall(EID, FID, (0, 0, 0)).expect("Getting specification version must always succeed.");
    SpecVersion {
        major: value.get_bits(24..31) as u8,
        minor: value.get_bits(0..24) as u32,
    }
}

/// Retrieve an identifier unique to the implementation.
pub fn impl_id() -> ImplId {
    use crate::{ecall, FunctionId};
    const FID: FunctionId = 0x1;
    let value = ecall(EID, FID, (0, 0, 0)).expect("Getting implementation ID must always succeed.");
    ImplId(value)
}

/// Retrieve the version of the implementation.
pub fn impl_version() -> ImplVersion {
    use crate::{ecall, FunctionId};
    const FID: FunctionId = 0x2;
    let value =
        ecall(EID, FID, (0, 0, 0)).expect("Getting implementation version must always succeed.");
    ImplVersion(value)
}

/// Retrieve the availability of a given extension by its ID.
pub fn probe_extension(eid: ExtensionId) -> ExtensionAvailability {
    use crate::{ecall, ArgValue, FunctionId};
    const FID: FunctionId = 0x3;
    let value =
        ecall(EID, FID, (eid as ArgValue, 0, 0)).expect("Probing extensions must always succeed.");
    if let Some(value) = NonZeroIsize::new(value) {
        ExtensionAvailability::Available(value)
    } else {
        ExtensionAvailability::Unavailable
    }
}

/// Retrieve an identifier unique to the machine vendor.
pub fn machine_vendor_id() -> MachineVendorId {
    use crate::{ecall, FunctionId};
    const FID: FunctionId = 0x4;
    let value = ecall(EID, FID, (0, 0, 0)).expect("Getting machine vendor ID must always succeed.");
    MachineVendorId(value)
}

/// Retrieve an identifier unique to the machine architecture.
pub fn machine_arch_id() -> MachineArchId {
    use crate::{ecall, FunctionId};
    const FID: FunctionId = 0x5;
    let value =
        ecall(EID, FID, (0, 0, 0)).expect("Getting machine architecture ID must always succeed.");
    MachineArchId(value)
}

/// Retrieve an identifier unique to the machine implementation.
pub fn machine_impl_id() -> MachineImplId {
    use crate::{ecall, FunctionId};
    const FID: FunctionId = 0x6;
    let value =
        ecall(EID, FID, (0, 0, 0)).expect("Getting machine implementation ID must always succeed.");
    MachineImplId(value)
}
