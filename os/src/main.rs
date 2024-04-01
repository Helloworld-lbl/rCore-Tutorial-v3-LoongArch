//! The main module and entrypoint
//!
//! Various facilities of the kernels are implemented as submodules. The most
//! important ones are:
//!
//! - [`trap`]: Handles all cases of switching from userspace to the kernel
//! - [`task`]: Task management
//! - [`syscall`]: System call handling and implementation
//!
//! The operating system also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality. (See its source code for
//! details.)
//!
//! We then call [`task::run_first_task()`] and for the first time go to
//! userspace.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;
use loongarch::time;
use crate::config::TICKS_PER_SEC;

// #[path = "boards/qemu.rs"]
// mod board;

#[macro_use]
mod console;
mod config;
mod lang_items;
mod loader;
mod uart;
mod sync;
pub mod syscall;
pub mod task;
// mod timer;
pub mod trap;
mod shutdown;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

/// clear BSS segment
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

/// the rust entry-point of os
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] Hello, world!");
    unsafe {
        let rd:usize;
        // core::arch::asm!("idle 0");
        core::arch::asm!("cpucfg {},{}", out(reg) rd, in(reg) 1);
        // core::arch::asm!("iocsrwr.b {},{}", in(reg) 0b00000000, in(reg) 0x01d8);
        // core::arch::asm!("iocsrwr.b {},{}", in(reg) 0b00000001, in(reg) 0x01d8);
        // core::arch::asm!("iocsrwr.b {},{}", in(reg) 0b00000011, in(reg) 0x01d8);
        println!("rd: {:b}", rd);
    }
    trap::init();
    loader::load_apps();
    time::init_trigger(TICKS_PER_SEC);
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}
