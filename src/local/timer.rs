use core::u8;
use core::convert::TryFrom;
use crate::local::{LocalApic, LocalApicRegister, LocalApicRegisterIndex};

bitflags! {
    pub struct TimerDivideConfigurationFlags: u32 {
        const ALL             = 0xffff_fffb;
        const DIVIDE_BITS_0_1 = 0x0000_0003;
        const DIVIDE_BITS_3   = 0x0000_0008;
        const DIVIDE_BITS     = 0x0000_000b;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LvtTimerDivideValue(pub u32);

impl From<TimerDivideConfigurationFlags> for LvtTimerDivideValue {
    fn from(flags: TimerDivideConfigurationFlags) -> Self {
        let a = (flags & TimerDivideConfigurationFlags::DIVIDE_BITS_0_1).bits();
        let b = (flags & TimerDivideConfigurationFlags::DIVIDE_BITS_3).bits();
        let c = (b >> 1) | a; // 3 contiguous bits

        let divisor = (2 as u8).rotate_left(c);

        LvtTimerDivideValue(divisor as u32)
    }
}

impl TryFrom<LvtTimerDivideValue> for TimerDivideConfigurationFlags {
    type Error = &'static str;

    fn try_from(divisor: LvtTimerDivideValue) -> Result<Self, Self::Error> {
        let bits = match divisor.0 {
            2   => Ok(0x0),
            4   => Ok(0x1),
            8   => Ok(0x2),
            16  => Ok(0x3),
            32  => Ok(0x8),
            64  => Ok(0x9),
            128 => Ok(0xa),
            1   => Ok(0xb),
            _   => Err("invalid timer TimerDivide configuration")
        }?;

        Self::from_bits(bits).ok_or("error converting TimerDivide configuration to bitfield")
    }
}

pub struct LvtTimerDivideConfigurationRegister;
impl LocalApicRegister for LvtTimerDivideConfigurationRegister {
    type Value = TimerDivideConfigurationFlags;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        Self::Value::from_bits_truncate(apic.read_reg_32(LocalApicRegisterIndex::TimerDivideConfiguration))
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::TimerDivideConfiguration, TimerDivideConfigurationFlags::from(value).bits());
    }
}

pub struct LvtTimerInitialCount(pub u32);
pub struct LvtTimerInitialCountRegister;
impl LocalApicRegister for LvtTimerInitialCountRegister {
    type Value = LvtTimerInitialCount;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        LvtTimerInitialCount(apic.read_reg_32(LocalApicRegisterIndex::TimerInitialCount))
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::TimerInitialCount, value.0);
    }
}

pub struct LvtTimerCurrentCount(pub u32);
pub struct LvtTimerCurrentCountRegister;
impl LocalApicRegister for LvtTimerCurrentCountRegister {
    type Value = LvtTimerCurrentCount;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        LvtTimerCurrentCount(apic.read_reg_32(LocalApicRegisterIndex::TimerCurrentCount))
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::TimerCurrentCount, value.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_flags_to_divisor() {
        assert_eq!(LvtTimerDivideValue::from(TimerDivideConfigurationFlags::from_bits(0xa).unwrap()), 
            LvtTimerDivideValue(128));
        assert_eq!(LvtTimerDivideValue::from(TimerDivideConfigurationFlags::from_bits(0xb).unwrap()), 
            LvtTimerDivideValue(1));
        assert_eq!(LvtTimerDivideValue::from(TimerDivideConfigurationFlags::from_bits(0x3).unwrap()), 
            LvtTimerDivideValue(16));
    }

    #[test]
    pub fn test_divisor_to_flags() {
        assert_eq!(TimerDivideConfigurationFlags::try_from(LvtTimerDivideValue(128)).expect("flags"), 
            TimerDivideConfigurationFlags::from_bits(0xa).unwrap());
        assert_eq!(TimerDivideConfigurationFlags::try_from(LvtTimerDivideValue(1)).expect("flags"), 
            TimerDivideConfigurationFlags::from_bits(0xb).unwrap());
        assert_eq!(TimerDivideConfigurationFlags::try_from(LvtTimerDivideValue(16)).expect("flags"), 
            TimerDivideConfigurationFlags::from_bits(0x3).unwrap());
    }
}