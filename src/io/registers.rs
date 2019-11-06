use crate::io::IoApic;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum IoApic32BitRegisterIndex {
    Id = 0x0,
    Version,
    ArbitrationId,
}

impl IoApic32BitRegisterIndex {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }

    pub fn as_u64(&self) -> u64 {
        self.as_u32() as u64
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IoApic64BitRegisterIndex {
    RedirectionEntry(u32)
}

impl IoApic64BitRegisterIndex {
    pub fn as_u32(&self) -> u32 {
        match *self {
            IoApic64BitRegisterIndex::RedirectionEntry(value) => value
        }
    }

    pub fn as_u64(&self) -> u64 {
        self.as_u32() as u64
    }
}

pub trait IoApicRegister {
    type Value;

    unsafe fn read(&self, apic: &dyn IoApic) -> Self::Value;
    unsafe fn write(&self, apic: &dyn IoApic, value: Self::Value);
}