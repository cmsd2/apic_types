pub mod arb;
pub mod id;
pub mod redirection;
pub mod registers;
pub mod version;

pub use arb::*;
pub use id::*;
pub use redirection::*;
pub use registers::*;
pub use version::*;

pub trait IoApic {
    unsafe fn read_reg_32(&self, index: IoApic32BitRegisterIndex) -> u32;
    unsafe fn write_reg_32(&self, index: IoApic32BitRegisterIndex, value: u32);

    unsafe fn read_reg_64(&self, index: IoApic64BitRegisterIndex) -> u64;
    unsafe fn write_reg_64(&self, index: IoApic64BitRegisterIndex, value: u64);
}
