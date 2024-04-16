//! Constants used in rCore

pub const USER_STACK_SIZE: usize = 4096 * 4;
pub const KERNEL_STACK_SIZE: usize = 4096 * 4;
pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;

/// physical address
pub const PALEN: usize = 48;
pub const VALEN: usize = 48;
pub const PPN_WIDTH: usize = PALEN - PAGE_SIZE_BITS;
pub const VPN_WIDTH: usize = VALEN - PAGE_SIZE_BITS;
pub const PAGE_SIZE: usize = 0x4000;
pub const PAGE_SIZE_BITS: usize = 0xe;
pub const TABLE_ENTRY_NUM: usize = 0x800;
pub const TABLE_ENTRY_NUM_BITS: usize = 0xb;

pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;

pub use crate::boards::{MEMORY_END, MMIO};

pub const TICKS_PER_SEC: usize = 100;
