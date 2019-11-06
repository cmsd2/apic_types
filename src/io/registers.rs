use crate::io::IoApic;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IoApicRegisterIndex {
    Id,
    Version,
    ArbitrationId,
    IrqRedirectionEntry(u32)
}

pub trait IoApicRegister {
    type Value;

    unsafe fn read(&self, apic: &dyn IoApic) -> Self::Value;
    unsafe fn write(&self, apic: &dyn IoApic, value: Self::Value);
}