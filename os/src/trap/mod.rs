//! Trap handling functionality
//!
//! For rCore, we have a single trap entry point, namely `__alltraps`. At
//! initialization in [`init()`], we set the `stvec` CSR to point to it.
//!
//! All traps go through `__alltraps`, which is defined in `trap.S`. The
//! assembly language code does just enough work restore the kernel space
//! context, ensuring that Rust code safely runs, and transfers control to
//! [`trap_handler()`].
//!
//! It then calls different functionality based on what exactly the exception
//! was. For example, timer interrupts trigger task preemption, and syscalls go
//! to [`syscall()`].

mod context;

use crate::syscall::syscall;
use core::arch::global_asm;
use loongarch::register::{eentry, estat::{self, Trap, Exception}};

global_asm!(include_str!("trap.S"));

/// initialize CSR `stvec` as the entry of `__alltraps`
pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        eentry::write(__alltraps as usize >> 12);
    }
}

#[no_mangle]
/// handle an interrupt, exception, or system call from user space
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let estat = estat::read(); // get trap cause
    // let stval = stval::read(); // get extra value
    match estat.cause() {
        Trap::Exception(Exception::SYS) => {
            cx.era += 4;
            cx.r[0] = syscall(cx.r[11], [cx.r[4], cx.r[5], cx.r[6]]) as usize;
        }
        Trap::Exception(Exception::PIS) => {
            println!("[kernel] Trap::Exception(Exception::PIS) Invalid store operation page exception in application, kernel killed it.");
            panic!("[kernel] Cannot continue!");
            // run_next_app();
        }
        Trap::Exception(Exception::IPE) => {
            println!("[kernel] Trap::Exception(Exception::IPE) Instruction privilege level exception in application, kernel killed it.");
            panic!("[kernel] Cannot continue!");
            // run_next_app();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}!",
                estat.cause()
            );
        }
    }
    cx
}

pub use context::TrapContext;
