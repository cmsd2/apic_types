use super::LocalApic;

#[repr(u32)]
pub enum LocalApicRegisterIndex {
    Id = 0x20,
    Version = 0x30,
    TaskPriority = 0x80,
    ArbitrationPriority = 0x90,
    ProcessPriority = 0xa0,
    EndOfInterrupt = 0xb0,
    RemoteRead = 0xc0,
    LocalDestination = 0xd0,
    DestinationFormat = 0xe0,
    SpuriousInterrupt = 0xf0,
    InService0 = 0x100,
    InService1 = 0x110,
    InService2 = 0x120,
    InService3 = 0x130,
    InService4 = 0x140,
    InService5 = 0x150,
    InService6 = 0x160,
    InService7 = 0x170,
    TriggerMode0 = 0x180,
    TriggerMode1 = 0x190,
    TriggerMode2 = 0x1a0,
    TriggerMode3 = 0x1b0,
    TriggerMode4 = 0x1c0,
    TriggerMode5 = 0x1d0,
    TriggerMode6 = 0x1e0,
    TriggerMode7 = 0x1f0,
    InterruptRequest0 = 0x200,
    InterruptRequest1 = 0x210,
    InterruptRequest2 = 0x220,
    InterruptRequest3 = 0x230,
    InterruptRequest4 = 0x240,
    InterruptRequest5 = 0x250,
    InterruptRequest6 = 0x260,
    InterruptRequest7 = 0x270,
    ErrorStatus = 0x280,
    LvtCmci = 0x2f0,
    InterruptCommand0 = 0x300,
    InterruptCommand1 = 0x310,
    LvtTimer = 0x320,
    LvtThermalSensor = 0x330,
    LvtPerfCounters = 0x340,
    LvtLINT0 = 0x350,
    LvtLINT1 = 0x360,
    LvtError = 0x370,
    InitialCount = 0x380,
    CurrentCount = 0x390,
    DivideConfiguration = 0x3e0,
    // ...
}

impl LocalApicRegisterIndex {
    pub fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn as_u64(self) -> u64 {
        u64::from(self.as_u32())
    }
}

pub trait LocalApicRegister {
    type Value;

    unsafe fn read(&self, apic: &dyn LocalApic) -> Self::Value;
    unsafe fn write(&self, apic: &dyn LocalApic, value: Self::Value);
}