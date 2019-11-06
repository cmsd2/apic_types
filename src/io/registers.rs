use crate::io::IoApic;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum IoApic32BitRegisterIndex {
    Id = 0x0,
    Version,
    ArbitrationId,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IoApic64BitRegisterIndex {
    RedirectionEntry(u32)
}

pub trait IoApicRegister {
    type Value;

    unsafe fn read(&self, apic: &dyn IoApic) -> Self::Value;
    unsafe fn write(&self, apic: &dyn IoApic, value: Self::Value);
}