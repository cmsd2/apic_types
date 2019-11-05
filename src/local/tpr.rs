use crate::local::{LocalApic, LocalApicRegister, LocalApicRegisterIndex, PriorityClass, PrioritySubClass};

bitflags! {
    pub struct TaskPriorityFlags: u32 {
        const SUB_CLASS = 0x0000000f;
        const CLASS     = 0x000000f0;
        const RESERVED  = 0xffffff00;
    }
}

impl TaskPriorityFlags {
    pub fn priority_class(&self) -> PriorityClass {
        PriorityClass((*self & TaskPriorityFlags::CLASS).bits() >> 4)
    }

    pub fn priority_sub_class(&self) -> PrioritySubClass {
        PrioritySubClass((*self & TaskPriorityFlags::SUB_CLASS).bits())
    }
}

impl From<PriorityClass> for TaskPriorityFlags {
    fn from(priority_class: PriorityClass) -> Self {
        Self::from_bits_truncate(priority_class.0 << 4) & TaskPriorityFlags::CLASS
    }
}

impl From<PrioritySubClass> for TaskPriorityFlags {
    fn from(priority_sub_class: PrioritySubClass) -> Self {
        Self::from_bits_truncate(priority_sub_class.0) & TaskPriorityFlags::SUB_CLASS
    }
}

pub struct TaskPriorityRegister;
impl LocalApicRegister for TaskPriorityRegister {
    type Value = TaskPriorityFlags;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        TaskPriorityFlags::from_bits(apic.read_reg_32(LocalApicRegisterIndex::TaskPriority)).unwrap()
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::TaskPriority, TaskPriorityFlags::from(value).bits());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_conversions() {
        let pc = PriorityClass(15);
        assert_eq!(pc, TaskPriorityFlags::from(pc).priority_class());

        let pc = PrioritySubClass(15);
        assert_eq!(pc, TaskPriorityFlags::from(pc).priority_sub_class());
    }
}