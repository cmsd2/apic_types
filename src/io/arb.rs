use crate::io::{IoApic, IoApicRegister, IoApic32BitRegisterIndex};

bitflags! {
    pub struct ArbitrationIdFlags: u32 {
        const ALL = 0xffffffff;
        const ID  = 0x0f000000;
    }
}

impl ArbitrationIdFlags {
    pub fn id(&self) -> ArbitrationId {
        ArbitrationId((*self & ArbitrationIdFlags::ID).bits() >> 24)
    }
}

impl From<ArbitrationId> for ArbitrationIdFlags {
    fn from(id: ArbitrationId) -> ArbitrationIdFlags {
        ArbitrationIdFlags::from_bits_truncate(id.0 << 24) & ArbitrationIdFlags::ID
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ArbitrationId(pub u32);

pub struct ArbitrationIdRegister;
impl IoApicRegister for ArbitrationIdRegister {
    type Value = ArbitrationId;

    unsafe fn read(&self, apic: &dyn IoApic) -> Self::Value {
        ArbitrationIdFlags::from_bits_truncate(apic.read_reg_32(IoApic32BitRegisterIndex::ArbitrationId)).id()
    }

    unsafe fn write(&self, _apic: &dyn IoApic, _value: Self::Value) {
        panic!("read only register");
    }
}