//! Constants used in rCore for qemu

pub const MEMORY_END: usize = 0xfffffff;

pub const MMIO: &[(usize, usize)] = &[
    (0x10000000, 0x5fffffff), // VIRT_TEST/RTC  in virt machine
];
