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
    unsafe fn read_reg_32(&self, index: IoApicRegisterIndex) -> u32;
    unsafe fn write_reg_32(&self, index: IoApicRegisterIndex, value: u32);

    unsafe fn read_reg_64(&self, index: IoApicRegisterIndex) -> u64;
    unsafe fn write_reg_64(&self, index: IoApicRegisterIndex, value: u64);
}