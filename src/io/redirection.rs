use core::result::Result;
use core::convert::TryFrom;

bitflags! {
    pub struct RedirectionEntryFlags: u64 {
        const VECTOR               = 0x0000_00ff;
        const DELIVERY_MODE        = 0x0000_0700;
        const DESTINATION_MODE     = 0x0000_0800;
        const DELIVERY_STATUS      = 0x0000_1000;
        const POLARITY             = 0x0000_2000;
        const REMOTE_IRR           = 0x0000_4000;
        const TRIGGER_MODE         = 0x0000_8000;
        const MASK                 = 0x0001_0000;
        const RESERVED             = 0x00fe_0000;
        const PHYSICAL_DESTINATION = 0x0f00_0000;
        const LOGICAL_DESTINATION  = 0xff00_0000;
    }
}

impl RedirectionEntryFlags {
    pub fn vector(&self) -> Vector {
        Vector((*self & RedirectionEntryFlags::VECTOR).bits() as u32)
    }

    pub fn mask(&self) -> Mask {
        if self.contains(RedirectionEntryFlags::MASK) {
            Mask::Masked
        } else {
            Mask::NotMasked
        }
    }

    pub fn trigger_mode(&self) -> TriggerMode {
        if self.contains(RedirectionEntryFlags::TRIGGER_MODE) {
            TriggerMode::Level
        } else {
            TriggerMode::Edge
        }
    }

    pub fn polarity(&self) -> Polarity {
        if self.contains(RedirectionEntryFlags::POLARITY) {
            Polarity::ActiveLow
        } else {
            Polarity::ActiveHigh
        }
    }

    pub fn delivery_status(&self) -> DeliveryStatus {
        if self.contains(RedirectionEntryFlags::DELIVERY_STATUS) {
            DeliveryStatus::SendPending
        } else {
            DeliveryStatus::Idle
        }
    }

    pub fn destination_mode(&self) -> DestinationMode {
        if self.contains(RedirectionEntryFlags::DESTINATION_MODE) {
            DestinationMode::Logical
        } else {
            DestinationMode::Physical
        }
    }

    pub fn delivery_mode(&self) -> DeliveryMode {
        DeliveryMode::try_from(((*self & RedirectionEntryFlags::DELIVERY_MODE).bits() >> 8) as u8).unwrap()
    }

    pub fn destination(&self) -> Destination {
        if self.destination_mode() == DestinationMode::Logical {
            Destination::Logical(((*self & RedirectionEntryFlags::LOGICAL_DESTINATION).bits() >> 24) as u8)
        } else {
            Destination::Physical(((*self & RedirectionEntryFlags::PHYSICAL_DESTINATION).bits() >> 24) as u8)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector(pub u32);

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Mask {
    NotMasked = 0x0,
    Masked
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum TriggerMode {
    Edge,
    Level
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Polarity {
    ActiveHigh = 0x0,
    ActiveLow
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum DeliveryStatus {
    Idle = 0x0,
    SendPending
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum DestinationMode {
    Physical = 0x0,
    Logical
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum DeliveryMode {
    Fixed = 0x0,
    LowestPriority,
    SMI,
    Reserved0,
    NMI,
    INIT,
    Reserved1,
    ExtINT,
}

impl TryFrom<u8> for DeliveryMode {
    type Error = &'static str;

    fn try_from(value: u8)  -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(DeliveryMode::Fixed),
            0x1 => Ok(DeliveryMode::LowestPriority),
            0x2 => Ok(DeliveryMode::SMI),
            0x3 => Ok(DeliveryMode::Reserved0),
            0x4 => Ok(DeliveryMode::NMI),
            0x5 => Ok(DeliveryMode::INIT),
            0x6 => Ok(DeliveryMode::Reserved1),
            0x7 => Ok(DeliveryMode::ExtINT),
            _ => Err("unrecognised delivery mode")
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Destination {
    Physical(u8),
    Logical(u8),
}

