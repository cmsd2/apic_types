use core::convert::TryFrom;
use core::result::Result;

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
    }
}

impl InterruptCommandFlags {
    pub fn delivery_mode(&self) -> IcrDeliveryMode {
        let bits = (*self & InterruptCommandFlags::DELIVERY_MODE).bits() >> 8;
        IcrDeliveryMode::try_from(bits as u8).expect("icr delivery mode")
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
