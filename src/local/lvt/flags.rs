use core::convert::TryFrom;

bitflags! {
    pub struct LvtFlags: u32 {
        const ALL                = 0xffff_ffff;
        const VECTOR             = 0x0000_00ff;
        const DELIVERY_MODE      = 0x0000_0700;
        const DELIVERY_STATUS    = 0x0000_1000;
        const INPUT_PIN_POLARITY = 0x0000_2000;
        const REMOTE_IRR         = 0x0000_4000;
        const TRIGGER_MODE       = 0x0000_8000;
        const MASK               = 0x0001_0000;
        const TIMER_MODE         = 0x0006_0000;
    }
}

impl LvtFlags {
    pub fn as_u32(&self) -> u32 {
        self.bits()
    }

    pub fn vector(&self) -> u32 {
        (*self & LvtFlags::VECTOR).bits()
    }

    pub fn delivery_mode(&self) -> LvtDeliveryMode {
        LvtDeliveryMode::try_from((*self & LvtFlags::DELIVERY_MODE).bits() >> 8)
            .expect("delivery mode")
    }

    pub fn delivery_status(&self) -> LvtDeliveryStatus {
        self.contains(LvtFlags::DELIVERY_STATUS).into()
    }

    pub fn input_pin_polarity(&self) -> LvtInputPinPolarity {
        self.contains(LvtFlags::INPUT_PIN_POLARITY).into()
    }

    pub fn remote_irr(&self) -> bool {
        self.contains(LvtFlags::REMOTE_IRR)
    }

    pub fn trigger_mode(&self) -> LvtTriggerMode {
        self.contains(LvtFlags::TRIGGER_MODE).into()
    }

    pub fn mask(&self) -> LvtMask {
        self.contains(LvtFlags::MASK).into()
    }

    pub fn timer_mode(&self) -> LvtTimerMode {
        LvtTimerMode::try_from((*self & LvtFlags::TIMER_MODE).bits() >> 16)
            .expect("timer mode")
    }
}

impl From<LvtDeliveryMode> for LvtFlags {
    fn from(mode: LvtDeliveryMode) -> Self {
        Self::from_bits_truncate(mode.as_u32() << 8)
    }
}

impl From<LvtDeliveryStatus> for LvtFlags {
    fn from(status: LvtDeliveryStatus) -> Self {
        Self::from_bits_truncate(status.as_u32() << 12)
    }
}

impl From<LvtInputPinPolarity> for LvtFlags {
    fn from(status: LvtInputPinPolarity) -> Self {
        Self::from_bits_truncate(status.as_u32() << 13)
    }
}

impl From<LvtTriggerMode> for LvtFlags {
    fn from(status: LvtTriggerMode) -> Self {
        Self::from_bits_truncate(status.as_u32() << 15)
    }
}

impl From<LvtMask> for LvtFlags {
    fn from(mask: LvtMask) -> Self {
        Self::from_bits_truncate(mask.as_u32() << 16)
    }
}

impl From<LvtTimerMode> for LvtFlags {
    fn from(mode: LvtTimerMode) -> LvtFlags {
        Self::from_bits_truncate(mode.as_u32() << 16) & LvtFlags::TIMER_MODE
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum LvtDeliveryStatus {
    Idle = 0x0,
    SendPending,
}

impl LvtDeliveryStatus {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_u32(self) -> u32 {
        self.as_u8() as u32
    }
}

impl From<bool> for LvtDeliveryStatus {
    fn from(b: bool) -> LvtDeliveryStatus {
        if b {
            LvtDeliveryStatus::SendPending
        } else {
            LvtDeliveryStatus::Idle
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum LvtInputPinPolarity {
    ActiveHigh = 0x0,
    ActiveLow,
}

impl LvtInputPinPolarity {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_u32(self) -> u32 {
        self.as_u8() as u32
    }
}

impl From<bool> for LvtInputPinPolarity {
    fn from(b: bool) -> LvtInputPinPolarity {
        if b {
            LvtInputPinPolarity::ActiveLow
        } else {
            LvtInputPinPolarity::ActiveHigh
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum LvtMask {
    NotMasked = 0x0,
    Masked,
}

impl LvtMask {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_u32(self) -> u32 {
        self.as_u8() as u32
    }
}

impl From<bool> for LvtMask {
    fn from(b: bool) -> LvtMask {
        if b {
            LvtMask::Masked
        } else {
            LvtMask::NotMasked
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum LvtTimerMode {
    OneShot = 0x0,
    Periodic,
    TSCDeadline,
}

impl LvtTimerMode {
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub fn as_u32(&self) -> u32 {
        self.as_u8() as u32
    }
}

impl TryFrom<u32> for LvtTimerMode {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(LvtTimerMode::OneShot),
            0x1 => Ok(LvtTimerMode::Periodic),
            0x2 => Ok(LvtTimerMode::TSCDeadline),
            _ => Err("invalid lvt timer mode")
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum LvtTriggerMode {
    Edge = 0x0,
    Level,
}

impl LvtTriggerMode {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_u32(self) -> u32 {
        self.as_u8() as u32
    }
}

impl From<bool> for LvtTriggerMode {
    fn from(b: bool) -> LvtTriggerMode {
        if b {
            LvtTriggerMode::Level
        } else {
            LvtTriggerMode::Edge
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum LvtDeliveryMode {
    Fixed = 0x0,
    Reserved1,
    SMI,
    Reserved2,
    NMI,
    INIT,
    Reserved3,
    ExtINT,
}

impl LvtDeliveryMode {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_u32(self) -> u32 {
        self.as_u8() as u32
    }
}

impl TryFrom<u32> for LvtDeliveryMode {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(LvtDeliveryMode::Fixed),
            0x1 => Ok(LvtDeliveryMode::Reserved1),
            0x2 => Ok(LvtDeliveryMode::SMI),
            0x3 => Ok(LvtDeliveryMode::Reserved2),
            0x4 => Ok(LvtDeliveryMode::NMI),
            0x5 => Ok(LvtDeliveryMode::INIT),
            0x6 => Ok(LvtDeliveryMode::Reserved3),
            0x7 => Ok(LvtDeliveryMode::ExtINT),
            _ => Err("invalid delivery mode")
        }
    }
}

impl TryFrom<u8> for LvtDeliveryMode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(value as u32)
    }
}
