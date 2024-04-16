//! map area and memory set, is implemented here.
//!
//! Every task or process has a memory_set to control its virtual memory.

mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
use address::{StepByOne, VPNRange};
pub use frame_allocator::{frame_alloc, FrameTracker};
pub use memory_set::remap_test;
pub use memory_set::{MapPermission, MemorySet, KERNEL_SPACE, TRAMPOLINE_SPACE};
pub use page_table::{translated_byte_buffer, translated_refmut, translated_str, PageTableEntry};
use page_table::{PTEFlags, PageTable};
use loongarch::register::{tlbrentry, pwcl, pwch, stlbps};

/// initiate heap allocator, frame allocator and kernel space
pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    init_tlb();
    KERNEL_SPACE.exclusive_access().activate();
    TRAMPOLINE_SPACE.exclusive_access().activate_trampoline();
}

fn init_tlb() {
    extern "C" {
        fn stlbrentry();
    } 
    tlbrentry::write_pa_to_tlbrentry(stlbrentry as usize);

    let mut pwcl = pwcl::read();
    let mut pwch = pwch::read();
    pwcl.set_ptbase(14);
    pwcl.set_ptwidth(11);
    // PMD
    pwcl.set_dir1_base(25);
    pwcl.set_dir1_width(11);
    // PGD
    pwch.set_dir3_base(36);
    pwch.set_dir3_width(11);
    pwcl.write();
    pwch.write();

    let mut stlbps = stlbps::read();
    stlbps.set_ps(0xe);
    stlbps.write();
}
