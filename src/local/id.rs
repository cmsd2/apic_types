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
    pub fn new_for_id(id: ApicId) -> IdFlags {
        let (value, mask) = match id {
            ApicId::Id4Bit(value) => (value, IdFlags::ID_4_BIT),
            ApicId::Id8Bit(value) => (value, IdFlags::ID_8_BIT),
        };

        IdFlags::from_bits_truncate(value << 24) & mask
    }

    pub fn id_4_bit(&self) -> ApicId {
        ApicId::Id4Bit((*self & IdFlags::ID_4_BIT).bits() >> 24)
    }

    pub fn id_8_bit(&self) -> ApicId {
        ApicId::Id8Bit((*self & IdFlags::ID_8_BIT).bits() >> 24)
    }
}

impl From<u32> for IdFlags {
    fn from(value: u32) -> Self {
        IdFlags::from_bits(value).unwrap()
    }
}

impl From<ApicId> for IdFlags {
    fn from(value: ApicId) -> Self {
        IdFlags::new_for_id(value)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ApicId {
    Id4Bit(u32),
    Id8Bit(u32),
}

pub struct Id4BitRegister;
impl LocalApicRegister for Id4BitRegister {
    type Value = ApicId;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        IdFlags::from(apic.read_reg_32(LocalApicRegisterIndex::Id)).id_4_bit()
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::Id, IdFlags::from(value).bits())
    }
}

pub struct Id8BitRegister;
impl LocalApicRegister for Id8BitRegister {
    type Value = ApicId;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        IdFlags::from(apic.read_reg_32(LocalApicRegisterIndex::Id)).id_8_bit()
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::Id, IdFlags::from(value).bits());
    }
}