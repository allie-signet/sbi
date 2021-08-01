//! The base extension contains all the required functions that any
//! conforming SBI implementation must implement. It permits the
//! supervisor to probe for information about the implementation.

use core::num::NonZeroUsize;

const EID: u32 = 0x10;

/// Denotes the version of the SBI specification that an implementation
/// conforms to.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct SpecVersion(pub u32);

impl SpecVersion {
    /// The major revision.
    pub fn major(&self) -> u32 {
        use bit_field::BitField;
        self.0.get_bits(24..31)
    }

    /// The minor revision.
    pub fn minor(&self) -> u32 {
        use bit_field::BitField;
        self.0.get_bits(0..24)
    }
}

/// A unique identifier for an implementation.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ImplId(pub usize);

impl ImplId {
    /// Checks whether the implementation is the Berkley Boot Loader.
    pub fn is_bbl(&self) -> bool {
        self.0 == 0x0
    }

    /// Checks whether the implementation is OpenSBI.
    pub fn is_opensbi(&self) -> bool {
        self.0 == 0x1
    }

    /// Checks whether the implementation is Xvisor.
    pub fn is_xvisor(&self) -> bool {
        self.0 == 0x2
    }

    /// Checks whether the implementation is KVM.
    pub fn is_kvm(&self) -> bool {
        self.0 == 0x3
    }

    /// Checks whether the implementation is RustSBI.
    pub fn is_rustsbi(&self) -> bool {
        self.0 == 0x4
    }

    /// Checks whether the implementation is Diosix.
    pub fn is_diosix(&self) -> bool {
        self.0 == 0x5
    }
}

/// A unique identifier for an machine vendor (`mvendorid`).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct MachineVendorId(pub u32);

impl MachineVendorId {
    /// The JEDEC bank.
    pub fn bank(&self) -> u32 {
        use bit_field::BitField;
        self.0.get_bits(7..32)
    }

    /// The JEDEC offset.
    pub fn offset(&self) -> u32 {
        use bit_field::BitField;
        self.0.get_bits(0..7)
    }
}

/// Denotes availability of an extension.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ExtensionAvailability {
    /// An available extension and optional extension-specific data.
    Available(NonZeroUsize),
    /// An unavailable extension.
    Unavailable,
}

/// Retrieve the version of the SBI specification the implementation
/// conforms to.
pub fn spec_version() -> SpecVersion {
    use crate::ecall;
    let ret = ecall(EID, 0x0, Default::default());
    assert_eq!(ret.error, 0x0);
    SpecVersion(ret.value as u32)
}

/// Retrieve an identifier unique to the implementation.
pub fn impl_id() -> ImplId {
    use crate::ecall;
    let ret = ecall(EID, 0x1, Default::default());
    assert_eq!(ret.error, 0x0);
    ImplId(ret.value)
}

/// Retrieve the version of the implementation.
pub fn impl_version() -> usize {
    use crate::ecall;
    let ret = ecall(EID, 0x2, Default::default());
    assert_eq!(ret.error, 0x0);
    ret.value
}

/// Retrieve the availability of a given extension by its ID.
pub fn probe_extension(eid: u32) -> ExtensionAvailability {
    use crate::ecall;
    let ret = ecall(EID, 0x3, [eid as usize, 0, 0, 0, 0, 0]);
    assert_eq!(ret.error, 0x0);
    if let Some(value) = NonZeroUsize::new(ret.value) {
        ExtensionAvailability::Available(value)
    } else {
        ExtensionAvailability::Unavailable
    }
}

/// Retrieve an identifier unique to the machine vendor.
pub fn machine_vendor_id() -> MachineVendorId {
    use crate::ecall;
    let ret = ecall(EID, 0x4, Default::default());
    assert_eq!(ret.error, 0x0);
    MachineVendorId(ret.value as u32)
}

/// Retrieve an identifier unique to the machine architecture.
pub fn machine_arch_id() -> usize {
    use crate::ecall;
    let ret = ecall(EID, 0x5, Default::default());
    assert_eq!(ret.error, 0x0);
    ret.value
}

/// Retrieve an identifier unique to the machine implementation.
pub fn machine_impl_id() -> usize {
    use crate::ecall;
    let ret = ecall(EID, 0x6, Default::default());
    assert_eq!(ret.error, 0x0);
    ret.value
}
