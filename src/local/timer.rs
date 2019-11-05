use core::u8;
use core::convert::TryFrom;
use crate::local::{LocalApic, LocalApicRegister, LocalApicRegisterIndex};

bitflags! {
    pub struct DivideConfigurationFlags: u32 {
        const ALL             = 0xffff_fffb;
        const DIVIDE_BITS_0_1 = 0x0000_0003;
        const DIVIDE_BITS_3   = 0x0000_0008;
        const DIVIDE_BITS     = 0x0000_000b;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LvtDivideValue(pub u32);

impl From<DivideConfigurationFlags> for LvtDivideValue {
    fn from(flags: DivideConfigurationFlags) -> Self {
        let a = (flags & DivideConfigurationFlags::DIVIDE_BITS_0_1).bits();
        let b = (flags & DivideConfigurationFlags::DIVIDE_BITS_3).bits();
        let c = (b >> 1) | a; // 3 contiguous bits

        let divisor = (2 as u8).rotate_left(c);

        LvtDivideValue(divisor as u32)
    }
}

impl TryFrom<LvtDivideValue> for DivideConfigurationFlags {
    type Error = &'static str;

    fn try_from(divisor: LvtDivideValue) -> Result<Self, Self::Error> {
        let bits = match divisor.0 {
            2   => Ok(0x0),
            4   => Ok(0x1),
            8   => Ok(0x2),
            16  => Ok(0x3),
            32  => Ok(0x8),
            64  => Ok(0x9),
            128 => Ok(0xa),
            1   => Ok(0xb),
            _   => Err("invalid timer divide configuration")
        }?;

        Self::from_bits(bits).ok_or("error converting divide configuration to bitfield")
    }
}

pub struct LvtDivideConfigurationRegister;
impl LocalApicRegister for LvtDivideConfigurationRegister {
    type Value = DivideConfigurationFlags;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        Self::Value::from_bits_truncate(apic.read_reg_32(LocalApicRegisterIndex::DivideConfiguration))
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::DivideConfiguration, DivideConfigurationFlags::from(value).bits());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_flags_to_divisor() {
        assert_eq!(LvtDivideValue::from(DivideConfigurationFlags::from_bits(0xa).unwrap()), 
            LvtDivideValue(128));
        assert_eq!(LvtDivideValue::from(DivideConfigurationFlags::from_bits(0xb).unwrap()), 
            LvtDivideValue(1));
        assert_eq!(LvtDivideValue::from(DivideConfigurationFlags::from_bits(0x3).unwrap()), 
            LvtDivideValue(16));
    }

    #[test]
    pub fn test_divisor_to_flags() {
        assert_eq!(DivideConfigurationFlags::try_from(LvtDivideValue(128)).expect("flags"), 
            DivideConfigurationFlags::from_bits(0xa).unwrap());
        assert_eq!(DivideConfigurationFlags::try_from(LvtDivideValue(1)).expect("flags"), 
            DivideConfigurationFlags::from_bits(0xb).unwrap());
        assert_eq!(DivideConfigurationFlags::try_from(LvtDivideValue(16)).expect("flags"), 
            DivideConfigurationFlags::from_bits(0x3).unwrap());
    }
}