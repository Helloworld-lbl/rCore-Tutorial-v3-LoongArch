//! Constants used in rCore

pub const USER_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;

/// physical address
pub const PALEN: usize = 47;
pub const VALEN: usize = 47;
pub const PPN_WIDTH: usize = PALEN - PAGE_SIZE_BITS;
pub const VPN_WIDTH: usize = VALEN - PAGE_SIZE_BITS;
pub const PAGE_SIZE: usize = 0x4000;
pub const PAGE_SIZE_BITS: usize = 0xe;
pub const TABLE_ENTRY_NUM: usize = 0x800;
pub const TABLE_ENTRY_NUM_BITS: usize = 0xb;

pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;
/// Return (bottom, top) of a kernel stack in kernel space.
pub fn kernel_stack_position(app_id: usize) -> (usize, usize) {
    let top = TRAMPOLINE - app_id * (KERNEL_STACK_SIZE + PAGE_SIZE);
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom, top)
}

pub use crate::board::{CLOCK_FREQ, MEMORY_END, MMIO};
