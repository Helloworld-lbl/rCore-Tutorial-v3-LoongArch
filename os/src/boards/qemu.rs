//! Constants used in rCore for qemu

// pub const CLOCK_FREQ: usize = 12500000;
pub const MEMORY_END: usize = 0xfffffff;

pub const MMIO: &[(usize, usize)] = &[
    (0x10000000, 0x6fffffff), // VIRT_TEST/RTC  in virt machine
];
