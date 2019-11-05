use crate::local::{LocalApic, LocalApicRegister, LocalApicRegisterIndex, PriorityClass, PrioritySubClass};

bitflags! {
    pub struct ProcessorPriorityFlags: u32 {
        const SUB_CLASS = 0x0000000f;
        const CLASS     = 0x000000f0;
        const RESERVED  = 0xffffff00;
    }
}

impl ProcessorPriorityFlags {
    pub fn priority_class(&self) -> PriorityClass {
        PriorityClass((*self & ProcessorPriorityFlags::CLASS).bits() >> 4)
    }

    pub fn priority_sub_class(&self) -> PrioritySubClass {
        PrioritySubClass((*self & ProcessorPriorityFlags::SUB_CLASS).bits())
    }
}

impl From<PriorityClass> for ProcessorPriorityFlags {
    fn from(priority_class: PriorityClass) -> Self {
        Self::from_bits_truncate(priority_class.0 << 4) & ProcessorPriorityFlags::CLASS
    }
}

impl From<PrioritySubClass> for ProcessorPriorityFlags {
    fn from(priority_sub_class: PrioritySubClass) -> Self {
        Self::from_bits_truncate(priority_sub_class.0) & ProcessorPriorityFlags::SUB_CLASS
    }
}

pub struct ProcessorPriorityRegister;
impl LocalApicRegister for ProcessorPriorityRegister {
    type Value = ProcessorPriorityFlags;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        ProcessorPriorityFlags::from_bits(apic.read_reg_32(LocalApicRegisterIndex::ProcessorPriority)).unwrap()
    }

    unsafe fn write(&self, _apic: &dyn LocalApic, _value: Self::Value) {
        panic!("processor priority is read-only");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_conversions() {
        let pc = PriorityClass(15);
        assert_eq!(pc, ProcessorPriorityFlags::from(pc).priority_class());

        let pc = PrioritySubClass(15);
        assert_eq!(pc, ProcessorPriorityFlags::from(pc).priority_sub_class());
    }
}