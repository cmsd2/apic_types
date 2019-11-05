
bitflags! {
    pub struct DestinationFormatFlags: u32 {
        const RESERVED = 0x0fffffff;
        const MODEL    = 0xf0000000;
    }
}

impl Default for DestinationFormatFlags {
    fn default() -> Self {
        Self::flat()
    }
}

impl DestinationFormatFlags {
    pub fn flat() -> Self {
        DestinationFormatFlags::MODEL | DestinationFormatFlags::RESERVED
    }

    pub fn cluster() -> Self {
        DestinationFormatFlags::RESERVED
    }
}