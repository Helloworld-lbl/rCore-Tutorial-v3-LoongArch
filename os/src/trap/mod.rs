//! to [`syscall()`].
mod context;

use crate::config::TRAMPOLINE;
use crate::syscall::syscall;
use crate::task::{
    current_user_token, exit_current_and_run_next, suspend_current_and_run_next
};
use core::arch::{asm, global_asm};
use loongarch::register::{
    eentry,
    estat::{
        self, 
        Trap, 
        Exception,
        Interrupt},
    ticlr::Ticlr,
    badv,
    };

global_asm!(include_str!("tlbr.S"));
global_asm!(include_str!("trap.S"));

/// initialize CSR `stvec` as the entry of `__alltraps`
pub fn init() {
    set_kernel_trap_entry();
}

fn set_kernel_trap_entry() {
    unsafe {
        eentry::write(trap_from_kernel as usize >> 12);
    }
}

fn set_user_trap_entry() {
    unsafe {
        eentry::write(TRAMPOLINE as usize >> 12);
    }
}

#[no_mangle]
/// handle an interrupt, exception, or system call from user space
pub fn trap_handler(cx: &mut TrapContext) -> ! {
    set_kernel_trap_entry();
    let estat = estat::read(); // get trap cause
    let badv = badv::read();
    match estat.cause() {
        Trap::Exception(Exception::SYS) => {
            cx.era += 4;
            cx.r[4] = syscall(cx.r[11], [cx.r[4], cx.r[5], cx.r[6]]) as usize;
        }
        Trap::Exception(Exception::PIS) => {
            println!("[kernel] Trap::Exception(Exception::PIS) Invalid store operation page exception in application, bad addr = {:#x}, kernel killed it.", badv.bits());
            exit_current_and_run_next(-2);
        }
        Trap::Exception(Exception::PIL) => {
            println!("[kernel] Trap::Exception(Exception::PIL) Invalid load operation page exception in application, bad addr = {:#x}, kernel killed it.", badv.bits());
            exit_current_and_run_next(-2);
        }
        Trap::Exception(Exception::IPE) => {
            println!("[kernel] Trap::Exception(Exception::IPE) Instruction privilege level exception in application, kernel killed it.");
            exit_current_and_run_next(-3);
        }
        Trap::Interrupt(Interrupt::TI) => {
            Ticlr::clear();
            suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}!",
                estat.cause()
            );
        }
    }
    unsafe { asm!("or $sp, $fp, $r0"); }
    trap_return();
}

#[no_mangle]
/// set the new addr of __restore asm function in TRAMPOLINE page,
/// set the reg a0 = trap_cx_ptr, reg a1 = phy addr of usr page table,
/// finally, jump to new addr of __restore asm function
pub fn trap_return() -> ! {
    set_user_trap_entry();
    let user_pgdl = current_user_token();
    extern "C" {
        fn __alltraps();
        fn __restore();
    }
    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
    unsafe {
        asm!(
            "ibar 0",
            "or $sp, $fp, $r0",
            "jirl $r0, {restore_va}, 0x0",             // jump to new addr of __restore asm function
            restore_va = in(reg) restore_va,
            in("$a0") user_pgdl,        // a0 = phy addr of usr page table
            options(noreturn),
        );
    }
}

#[no_mangle]
/// Unimplement: traps/interrupts/exceptions from kernel mode
/// Todo: Chapter 9: I/O device
pub fn trap_from_kernel() -> ! {
    panic!("a trap {:?} from kernel!", estat::read().cause());
}

pub use context::TrapContext;
