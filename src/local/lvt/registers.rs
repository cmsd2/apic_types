
use crate::local::{LocalApic, LocalApicRegister, LocalApicRegisterIndex};
use super::{LvtFlags};

pub struct LvtTimerRegister;
impl LocalApicRegister for LvtTimerRegister {
    type Value = LvtFlags;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        Self::Value::from_bits_truncate(apic.read_reg_32(LocalApicRegisterIndex::LvtTimer))
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::LvtTimer, value.bits());
    }
}

pub struct LvtCmciRegister;
impl LocalApicRegister for LvtCmciRegister {
    type Value = LvtFlags;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        Self::Value::from_bits_truncate(apic.read_reg_32(LocalApicRegisterIndex::LvtCmci))
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::LvtCmci, value.bits());
    }
}

pub struct LvtLint0Register;
impl LocalApicRegister for LvtLint0Register {
    type Value = LvtFlags;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        Self::Value::from_bits_truncate(apic.read_reg_32(LocalApicRegisterIndex::LvtLINT0))
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::LvtLINT0, value.bits());
    }
}

pub struct LvtLint1Register;
impl LocalApicRegister for LvtLint1Register {
    type Value = LvtFlags;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        Self::Value::from_bits_truncate(apic.read_reg_32(LocalApicRegisterIndex::LvtLINT1))
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::LvtLINT1, value.bits());
    }
}

pub struct LvtErrorRegister;
impl LocalApicRegister for LvtErrorRegister {
    type Value = LvtFlags;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        Self::Value::from_bits_truncate(apic.read_reg_32(LocalApicRegisterIndex::LvtError))
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::LvtError, value.bits());
    }
}

pub struct LvtPerfCountersRegister;
impl LocalApicRegister for LvtPerfCountersRegister {
    type Value = LvtFlags;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        Self::Value::from_bits_truncate(apic.read_reg_32(LocalApicRegisterIndex::LvtPerfCounters))
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::LvtPerfCounters, value.bits());
    }
}

pub struct LvtThermalSensorRegister;
impl LocalApicRegister for LvtThermalSensorRegister {
    type Value = LvtFlags;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        Self::Value::from_bits_truncate(apic.read_reg_32(LocalApicRegisterIndex::LvtThermalSensor))
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::LvtThermalSensor, value.bits());
    }
}
