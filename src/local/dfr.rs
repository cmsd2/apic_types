use crate::local::{LocalApic, LocalApicRegister, LocalApicRegisterIndex};

bitflags! {
    pub struct DestinationFormatFlags: u32 {
        const RESERVED = 0x0fffffff;
        const MODEL    = 0xf0000000;
    }
}

impl DestinationFormatFlags {
    pub fn model(&self) -> DestinationFormatModel {
        if self.contains(DestinationFormatFlags::MODEL) {
            DestinationFormatModel::Flat
        } else {
            DestinationFormatModel::Cluster
        }
    }

    pub fn flat() -> Self {
        DestinationFormatFlags::MODEL | DestinationFormatFlags::RESERVED
    }

    pub fn cluster() -> Self {
        DestinationFormatFlags::RESERVED
    }
}

impl Default for DestinationFormatFlags {
    fn default() -> Self {
        Self::flat()
    }
}

impl From<DestinationFormatModel> for DestinationFormatFlags {
    fn from(model: DestinationFormatModel) -> DestinationFormatFlags {
        match model {
            DestinationFormatModel::Flat => DestinationFormatFlags::flat(),
            DestinationFormatModel::Cluster => DestinationFormatFlags::cluster(),
        }
    }
}

impl From<DestinationFormatFlags> for DestinationFormatModel {
    fn from(flags: DestinationFormatFlags) -> DestinationFormatModel {
        flags.model()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum DestinationFormatModel {
    Cluster = 0x0,
    Flat = 0xf,
}

impl DestinationFormatModel {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

pub struct LogicalDestinationRegister;
impl LocalApicRegister for LogicalDestinationRegister {
    type Value = DestinationFormatFlags;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        DestinationFormatFlags::from_bits_truncate(apic.read_reg_32(LocalApicRegisterIndex::DestinationFormat))
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::DestinationFormat, DestinationFormatFlags::from(value).bits());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_conversions() {
        let model = DestinationFormatModel::Flat;
        assert_eq!(DestinationFormatFlags::from(model), DestinationFormatFlags::flat());
        assert_eq!(model, DestinationFormatModel::from(DestinationFormatFlags::flat()));

        let model = DestinationFormatModel::Cluster;
        assert_eq!(DestinationFormatFlags::from(model), DestinationFormatFlags::cluster());
        assert_eq!(model, DestinationFormatModel::from(DestinationFormatFlags::cluster()));
    }
}