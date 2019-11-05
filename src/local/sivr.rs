use super::LocalApic;
use super::registers::{LocalApicRegister, LocalApicRegisterIndex};

bitflags! {
    pub struct SivrFlags: u32 {
        const VECTOR = 0b0_0000_11111111;
        const ENABLE = 0b0_0001_00000000;
        const FPC    = 0b0_0010_00000000;
        const EOI    = 0b1_0000_00000000;
        const UNUSED = 0xffffe000;
    }
}

pub struct SpuriousInterruptVectorRegister;
impl LocalApicRegister for SpuriousInterruptVectorRegister {
    type Value = SivrFlags;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        Self::Value::from_bits_truncate(apic.read_reg_32(LocalApicRegisterIndex::SpuriousInterrupt))
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::SpuriousInterrupt, value.bits());
    }
}