use super::LocalApic;
use super::registers::{LocalApicRegister, LocalApicRegisterIndex};

bitflags! {
    pub struct VersionFlags: u32 {
        const VERSION          = 0x000000ff;
        const RESERVED         = 0x0000ff00;
        const MAX_LVT_ENTRY    = 0x00ff0000;
        const CAN_SUPPRESS_EOI = 0x01000000;
        const RESERVED2        = 0xfe000000;
    }
}

impl VersionFlags {
    pub fn version(&self) -> u8 {
        (*self & VersionFlags::VERSION).bits() as u8
    }

    pub fn lvt_entries(&self) -> u32 {
        ((*self & VersionFlags::MAX_LVT_ENTRY).bits() >> 16) + 1
    }

    pub fn can_suppress_eoi(&self) -> bool {
        self.contains(VersionFlags::CAN_SUPPRESS_EOI)
    }
}

pub struct VersionRegister;
impl LocalApicRegister for VersionRegister {
    type Value = VersionFlags;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        Self::Value::from_bits_truncate(apic.read_reg_32(LocalApicRegisterIndex::Version))
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::Version, value.bits());
    }
}
