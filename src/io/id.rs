use crate::io::{IoApic, IoApicRegister, IoApicRegisterIndex};

bitflags! {
    pub struct IdFlags: u32 {
        const ALL = 0xffffffff;
        const ID  = 0x0f000000;
    }
}

impl IdFlags {
    pub fn id(&self) -> ApicId {
        ApicId((*self & IdFlags::ID).bits() >> 24)
    }
}

impl From<ApicId> for IdFlags {
    fn from(id: ApicId) -> IdFlags {
        IdFlags::from_bits_truncate(id.0 << 24) & IdFlags::ID
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ApicId(pub u32);

pub struct IdRegister;
impl IoApicRegister for IdRegister {
    type Value = ApicId;

    unsafe fn read(&self, apic: &dyn IoApic) -> Self::Value {
        IdFlags::from_bits_truncate(apic.read_reg_32(IoApicRegisterIndex::Id)).id()
    }

    unsafe fn write(&self, apic: &dyn IoApic, value: Self::Value) {
        apic.write_reg_32(IoApicRegisterIndex::Id, IdFlags::from(value).bits());
    }
}