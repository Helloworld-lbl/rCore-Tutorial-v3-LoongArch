#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(naked_functions)]

#[macro_use]
mod console;
mod lang_items;
mod uart;
mod logging;
mod multicore;

use core::{arch::global_asm, sync::atomic::Ordering};
use log::*;

use crate::multicore::{ start_secondary_cpus, ENTERED_CPUS };

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext(); // begin addr of text segment
        fn etext(); // end addr of text segment
        fn srodata(); // start addr of Read-Only data segment
        fn erodata(); // end addr of Read-Only data ssegment
        fn sdata(); // start addr of data segment
        fn edata(); // end addr of data segment
        fn sbss(); // start addr of BSS segment
        fn ebss(); // end addr of BSS segment
        fn boot_stack_lower_bound(); // stack lower bound
        fn boot_stack_top(); // stack top
    }
    clear_bss();
    logging::init();
    println!("[kernel] Hello, world!");

    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as usize,
        etext as usize
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    warn!(
        "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as usize, boot_stack_lower_bound as usize
    );
    error!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);

    // CI autotest success: sbi::shutdown(false)
    // CI autotest failed : sbi::shutdown(true)

    start_secondary_cpus(0);

    panic!("Shutdown machine!");
}

#[no_mangle]
pub extern "C" fn rust_main_secondary(cpu_id: usize) -> ! {
    ENTERED_CPUS.fetch_add(1, Ordering::Relaxed);
    info!("Secondary CPU {:x} started.", cpu_id);
    println!("[kernel][CPU{:x}] Hello, world!", cpu_id);

    panic!("Shutdown machine from CPU{:x}!", cpu_id);
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0)}
    });
}
