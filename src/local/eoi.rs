use crate::local::{LocalApic, LocalApicRegister, LocalApicRegisterIndex};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Eoi(pub u32);
pub struct EoiRegister;
impl LocalApicRegister for EoiRegister {
    type Value = Eoi;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        Eoi(apic.read_reg_32(LocalApicRegisterIndex::EndOfInterrupt)).into()
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::EndOfInterrupt, value.0);
    }
}