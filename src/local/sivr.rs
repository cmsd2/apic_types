use crate::local::{LocalApic, LocalApicRegister, LocalApicRegisterIndex, InterruptVector};

bitflags! {
    pub struct SivrFlags: u32 {
        const VECTOR                   = 0b0000_0000_0000_11111111;
        const APIC_ENABLE              = 0b0000_0000_0001_00000000;
        const FOCUS_PROCESSOR_CHECKING = 0b0000_0000_0010_00000000;
        const EOI_BROADCAST_SUPRESSION = 0b0000_0001_0000_00000000;
        const UNUSED                   = 0b1111_1110_1100_00000000;
    }
}

impl SivrFlags {
    pub fn is_enabled(&self) -> bool {
        self.contains(SivrFlags::APIC_ENABLE)
    }

    pub fn is_focus_processor_checked(&self) -> bool {
        self.contains(SivrFlags::FOCUS_PROCESSOR_CHECKING)
    }

    pub fn is_eoi_broadcast_supressed(&self) -> bool {
        self.contains(SivrFlags::EOI_BROADCAST_SUPRESSION)
    }

    pub fn vector(&self) -> InterruptVector {
        InterruptVector((*self & SivrFlags::VECTOR).bits())
    }
}

pub struct SpuriousInterruptVectorRegister;
impl LocalApicRegister for SpuriousInterruptVectorRegister {
    type Value = SivrFlags;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        Self::Value::from_bits_truncate(apic.read_reg_32(LocalApicRegisterIndex::SpuriousInterrupt))
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::SpuriousInterrupt, value.bits());
    }
}