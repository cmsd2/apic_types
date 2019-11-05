use crate::local::{LocalApic, LocalApicRegister, LocalApicRegisterIndex};

bitflags! {
    pub struct LogicalDestinationFlags: u32 {
        const RESERVED        = 0x00ffffff;
        const LOGICAL_APIC_ID = 0xff000000;
    }
}

impl LogicalDestinationFlags {
    pub fn logical_apic_id(&self) -> LogicalApicId {
        LogicalApicId((*self & LogicalDestinationFlags::LOGICAL_APIC_ID).bits() >> 24)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LogicalApicId(pub u32);

impl From<LogicalDestinationFlags> for LogicalApicId {
    fn from(flags: LogicalDestinationFlags) -> Self {
        flags.logical_apic_id()
    }
}

impl From<LogicalApicId> for LogicalDestinationFlags {
    fn from(id: LogicalApicId) -> LogicalDestinationFlags {
        LogicalDestinationFlags::from_bits(id.0 << 24).unwrap()
    }
}

pub struct LogicalDestinationRegister;
impl LocalApicRegister for LogicalDestinationRegister {
    type Value = LogicalDestinationFlags;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        LogicalDestinationFlags::from_bits_truncate(apic.read_reg_32(LocalApicRegisterIndex::LogicalDestination))
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::LogicalDestination, LogicalDestinationFlags::from(value).bits());
    }
}