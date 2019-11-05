
bitflags! {
    pub struct LogicalDestinationFlags: u32 {
        const RESERVED        = 0x00ffffff;
        const LOGICAL_APIC_ID = 0xff000000;
    }
}
