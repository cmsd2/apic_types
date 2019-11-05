pub mod apr;
pub mod dfr;
pub mod eoi;
pub mod esr;
pub mod icr;
pub mod id;
pub mod irr;
pub mod isr;
pub mod ldr;
pub mod lvt;
pub mod ppr;
pub mod sivr;
pub mod timer;
pub mod tmr;
pub mod tpr;
pub mod version;
pub mod registers;

pub use apr::*;
pub use dfr::*;
pub use eoi::*;
pub use esr::*;
pub use icr::*;
pub use id::*;
pub use irr::*;
pub use isr::*;
pub use ldr::*;
pub use lvt::*;
pub use ppr::*;
pub use sivr::*;
pub use timer::*;
pub use tmr::*;
pub use tpr::*;
pub use version::*;
pub use registers::*;


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PriorityClass(pub u32);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PrioritySubClass(pub u32);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InterruptVector(pub u32);

impl InterruptVector {
    pub fn priority_class(&self) -> PriorityClass {
        PriorityClass((self.0 & 0xf0) >> 4)
    }

    pub fn priority_sub_class(&self) -> PrioritySubClass {
        PrioritySubClass(self.0 & 0xf)
    }
}

pub trait LocalApic {
    unsafe fn read_reg_32(&self, index: LocalApicRegisterIndex) -> u32;
    unsafe fn write_reg_32(&self, index: LocalApicRegisterIndex, value: u32);
}
