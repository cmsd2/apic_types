use crate::io::{IoApic, IoApicRegister, IoApicRegisterIndex};

bitflags! {
    pub struct VersionFlags: u32 {
        const ALL                = 0xffff_ffff;
        const VERSION            = 0x0000_00ff;
        const MAX_REDIRECT_ENTRY = 0x00ff_0000;
    }
}

impl VersionFlags {
    pub fn version(&self) -> Version {
        Version((*self & VersionFlags::VERSION).bits())
    }

    pub fn max_redirect_entry(&self) -> u32 {
        (*self & VersionFlags::MAX_REDIRECT_ENTRY).bits() >> 16
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Version(pub u32);

pub struct VersionRegister;
impl IoApicRegister for VersionRegister {
    type Value = VersionFlags;

    unsafe fn read(&self, apic: &dyn IoApic) -> Self::Value {
        VersionFlags::from_bits_truncate(apic.read_reg_32(IoApicRegisterIndex::Version))
    }

    unsafe fn write(&self, _apic: &dyn IoApic, _value: Self::Value) {
        panic!("read only register");
    }
}