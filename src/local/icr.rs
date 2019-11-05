use crate::local::{LocalApic, LocalApicRegister, LocalApicRegisterIndex};
use core::convert::TryFrom;
use core::result::Result;
use crate::local::InterruptVector;

bitflags! {
    pub struct InterruptCommandFlags: u64 {
        const VECTOR = 0xff;
        const DELIVERY_MODE = 0x700;
        const DELIVERY_MODE_LOWEST_PRIORITY = 0x100;
        const DELIVERY_MODE_SMI = 0x200;
        const DELIVERY_MODE_RESERVED = 0x300;
        const DELIVERY_MODE_NMI = 0x400;
        const DELIVERY_MODE_INIT = 0x500;
        const DELIVERY_MODE_START_UP = 0x600;
        const DELIVERY_MODE_RESERVED2 = 0x700;
        const DESTINATION_MODE = 0x800;
        const DELIVERY_STATUS = 0x1000;
        const RESERVED = 0x2000;
        const LEVEL = 0x4000;
        const TRIGGER_MODE = 0x8000;
        const RESERVED2 = 0x30000;
        const DESTINATION_SHORTHAND = 0xc0000;
        const RESERVED3 = 0x00ffffff_fff00000;
        const DESTINATION = 0xff000000_00000000;
        const LOW_BITS = 0xffffffff;
        const HIGH_BITS = 0xffffffff_00000000;
    }
}

impl InterruptCommandFlags {
    pub fn delivery_mode(&self) -> IcrDeliveryMode {
        let bits = (*self & InterruptCommandFlags::DELIVERY_MODE).bits() >> 8;
        IcrDeliveryMode::try_from(bits as u8).expect("icr delivery mode")
    }

    pub fn vector(&self) -> InterruptVector {
        InterruptVector((*self & InterruptCommandFlags::VECTOR).bits() as u32)
    }

    pub fn low_word(&self) -> u32 {
        self.bits() as u32
    }

    pub fn high_word(&self) -> u32 {
        (self.bits() >> 32) as u32
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum IcrDeliveryMode {
    Fixed = 0x0,
    LowestPriority,
    SMI,
    Reserved,
    NMI,
    INIT,
    StartUp,
    Reserved2,
}

impl IcrDeliveryMode {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_u64(self) -> u64 {
        self.as_u8() as u64
    }

    pub fn as_flags(self) -> InterruptCommandFlags {
        InterruptCommandFlags::from_bits(self.as_u64() << 8).unwrap()
    }
}

impl TryFrom<u8> for IcrDeliveryMode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(IcrDeliveryMode::Fixed),
            0x1 => Ok(IcrDeliveryMode::LowestPriority),
            0x2 => Ok(IcrDeliveryMode::SMI),
            0x3 => Ok(IcrDeliveryMode::Reserved),
            0x4 => Ok(IcrDeliveryMode::NMI),
            0x5 => Ok(IcrDeliveryMode::INIT),
            0x6 => Ok(IcrDeliveryMode::StartUp),
            0x7 => Ok(IcrDeliveryMode::Reserved2),
            _ => Err("invalid icr delivery mode")
        }
    }
}

pub struct InterruptCommandRegister;
impl LocalApicRegister for InterruptCommandRegister {
    type Value = InterruptCommandFlags;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        let low = apic.read_reg_32(LocalApicRegisterIndex::InterruptCommand0);
        let high = apic.read_reg_32(LocalApicRegisterIndex::InterruptCommand1);

        Self::Value::from_bits(((high as u64) << 32) | low as u64).unwrap()
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        let low = value.low_word();
        let high = value.high_word();

        apic.write_reg_32(LocalApicRegisterIndex::InterruptCommand0, low);
        apic.write_reg_32(LocalApicRegisterIndex::InterruptCommand1, high);
    }
}