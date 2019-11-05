pub mod apr;
pub mod dfr;
pub mod esr;
pub mod icr;
pub mod id;
pub mod irr;
pub mod isr;
pub mod ldr;
pub mod lvt;
pub mod sivr;
pub mod timer;
pub mod tmr;
pub mod version;
pub mod registers;

pub use apr::*;
pub use dfr::*;
pub use esr::*;
pub use icr::*;
pub use id::*;
pub use irr::*;
pub use isr::*;
pub use ldr::*;
pub use lvt::*;
pub use sivr::*;
pub use timer::*;
pub use tmr::*;
pub use version::*;
pub use registers::*;

pub trait LocalApic {
    unsafe fn read_reg_32(&self, index: LocalApicRegisterIndex) -> u32;
    unsafe fn write_reg_32(&self, index: LocalApicRegisterIndex, value: u32);
}
