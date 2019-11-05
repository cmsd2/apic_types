use super::LocalApic;
use super::registers::{LocalApicRegister, LocalApicRegisterIndex};

bitflags! {
    pub struct IdFlags: u32 {
        const ALL       = 0xffff_ffff;
        const ID_4_BIT  = 0x0f00_0000;
        const ID_8_BIT  = 0xff00_0000;
        // read 32 bit addr for x2 mode from MSR 802h
    }
}

impl IdFlags {
    pub fn new_4_bit(id: Id4Bit) -> IdFlags {
        IdFlags::from_bits((id.0 & 0xf) << 24).unwrap()
    }

    pub fn new_8_bit(id: Id8Bit) -> IdFlags {
        IdFlags::from_bits((id.0 & 0xff) << 24).unwrap()
    }
}

impl Into<Id4Bit> for IdFlags {
    fn into(self) -> Id4Bit {
        let bits = (self & IdFlags::ID_4_BIT).bits() >> 24;
        Id4Bit(bits)
    }
}

impl Into<Id8Bit> for IdFlags {
    fn into(self) -> Id8Bit {
        let bits = (self & IdFlags::ID_8_BIT).bits() >> 24;
        Id8Bit(bits)
    }
}

impl From<u32> for IdFlags {
    fn from(value: u32) -> Self {
        IdFlags::from_bits(value).unwrap()
    }
}

impl From<Id4Bit> for IdFlags {
    fn from(value: Id4Bit) -> Self {
        IdFlags::new_4_bit(value)
    }
}

impl From<Id8Bit> for IdFlags {
    fn from(value: Id8Bit) -> Self {
        IdFlags::new_8_bit(value)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Id4Bit(pub u32);
pub struct Id4BitRegister;
impl LocalApicRegister for Id4BitRegister {
    type Value = Id4Bit;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        IdFlags::from(apic.read_reg_32(LocalApicRegisterIndex::Id)).into()
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::Id, IdFlags::from(value).bits());
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Id8Bit(pub u32);
pub struct Id8BitRegister;
impl LocalApicRegister for Id8BitRegister {
    type Value = Id8Bit;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        IdFlags::from(apic.read_reg_32(LocalApicRegisterIndex::Id)).into()
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::Id, IdFlags::from(value).bits());
    }
}