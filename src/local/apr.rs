
bitflags! {
    pub struct ArbitrationPriorityFlags: u32 {
        const SUB_CLASS = 0x0f;
        const CLASS     = 0xf0;
        const RESERVED  = 0xffffff00;
    }
}