//! Constants used in rCore for qemu

pub const MEMORY_END: usize = 0xfffffff;

pub const MMIO: &[(usize, usize)] = &[
    (0x10000000, 0x6fffffff),
    // (0x40040000, 0x3fff),
    // (0x0010_0000, 0x00_2000), // VIRT_TEST/RTC  in virt machine
    // (0x1000_1000, 0x00_1000), // Virtio Block in virt machine
];

// pub type BlockDeviceImpl = crate::drivers::block::VirtIOBlock;
