use crate::local::{LocalApic, LocalApicRegister, LocalApicRegisterIndex};

bitflags! {
    pub struct ArbitrationPriorityFlags: u32 {
        const SUB_CLASS = 0x0000000f;
        const CLASS     = 0x000000f0;
        const RESERVED  = 0xffffff00;
    }
}

impl ArbitrationPriorityFlags {
    pub fn priority_class(&self) -> ArbitrationPriorityClass {
        ArbitrationPriorityClass((*self & ArbitrationPriorityFlags::CLASS).bits() >> 4)
    }

    pub fn priority_sub_class(&self) -> ArbitrationPrioritySubClass {
        ArbitrationPrioritySubClass((*self & ArbitrationPriorityFlags::SUB_CLASS).bits())
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ArbitrationPrioritySubClass(pub u32);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ArbitrationPriorityClass(pub u32);

impl From<ArbitrationPriorityClass> for ArbitrationPriorityFlags {
    fn from(priority_class: ArbitrationPriorityClass) -> Self {
        Self::from_bits_truncate(priority_class.0 << 4) & ArbitrationPriorityFlags::CLASS
    }
}

impl From<ArbitrationPrioritySubClass> for ArbitrationPriorityFlags {
    fn from(priority_sub_class: ArbitrationPrioritySubClass) -> Self {
        Self::from_bits_truncate(priority_sub_class.0) & ArbitrationPriorityFlags::SUB_CLASS
    }
}

pub struct ArbitrationPriorityRegister;
impl LocalApicRegister for ArbitrationPriorityRegister {
    type Value = ArbitrationPriorityFlags;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        ArbitrationPriorityFlags::from_bits(apic.read_reg_32(LocalApicRegisterIndex::ArbitrationPriority)).unwrap()
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::ArbitrationPriority, ArbitrationPriorityFlags::from(value).bits());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_conversions() {
        let pc = ArbitrationPriorityClass(15);
        assert_eq!(pc, ArbitrationPriorityFlags::from(pc).priority_class());

        let pc = ArbitrationPrioritySubClass(15);
        assert_eq!(pc, ArbitrationPriorityFlags::from(pc).priority_sub_class());
    }
}