use crate::local::{LocalApic, LocalApicRegister, LocalApicRegisterIndex, PriorityClass, PrioritySubClass};

bitflags! {
    pub struct ArbitrationPriorityFlags: u32 {
        const SUB_CLASS = 0x0000000f;
        const CLASS     = 0x000000f0;
        const RESERVED  = 0xffffff00;
    }
}

impl ArbitrationPriorityFlags {
    pub fn priority_class(&self) -> PriorityClass {
        PriorityClass((*self & ArbitrationPriorityFlags::CLASS).bits() >> 4)
    }

    pub fn priority_sub_class(&self) -> PrioritySubClass {
        PrioritySubClass((*self & ArbitrationPriorityFlags::SUB_CLASS).bits())
    }
}

impl From<PriorityClass> for ArbitrationPriorityFlags {
    fn from(priority_class: PriorityClass) -> Self {
        Self::from_bits_truncate(priority_class.0 << 4) & ArbitrationPriorityFlags::CLASS
    }
}

impl From<PrioritySubClass> for ArbitrationPriorityFlags {
    fn from(priority_sub_class: PrioritySubClass) -> Self {
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
        let pc = PriorityClass(15);
        assert_eq!(pc, ArbitrationPriorityFlags::from(pc).priority_class());

        let pc = PrioritySubClass(15);
        assert_eq!(pc, ArbitrationPriorityFlags::from(pc).priority_sub_class());
    }
}