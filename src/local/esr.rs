use super::{LocalApic, LocalApicRegister, LocalApicRegisterIndex};

bitflags! {
    pub struct ErrorStatusFlags: u32 {
        const ALL = 0xffff_ffff;
        const SEND_CHECKSUM_ERROR = 0x1;
        const RECEIVE_CHECKSUM_ERROR = 0x2;
        const SEND_ACCEPT_ERROR = 0x4;
        const RECEIVE_ACCEPT_ERROR = 0x8;
        const REDIRECTABLE_IPI = 0x10;
        const SEND_ILLEGAL_VECTOR = 0x20;
        const RECEIVED_ILLEGAL_VECTOR = 0x40;
        const ILLEGAL_REGISTER_ADDRESS = 0x80;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum ErrorStatus {
    SendChecksumError = 0x1,
    ReceiveChecksumError = 0x2,
    SendAcceptError = 0x4,
    ReceiveAcceptError = 0x8,
    RedirectableIPI = 0x10,
    SendIllegalVector = 0x20,
    ReceivedIllegalVector = 0x40,
    IllegalRegisterAddress = 0x80,
}

impl ErrorStatus {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl From<ErrorStatus> for ErrorStatusFlags {
    fn from(error_status: ErrorStatus) -> Self {
        Self::from_bits_truncate(error_status.as_u32())
    }
}

pub struct ErrorStatusRegister;
impl LocalApicRegister for ErrorStatusRegister {
    type Value = ErrorStatusFlags;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value {
        Self::Value::from_bits_truncate(apic.read_reg_32(LocalApicRegisterIndex::ErrorStatus))
    }

    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value) {
        apic.write_reg_32(LocalApicRegisterIndex::ErrorStatus, Self::Value::from(value).bits());
    }
}