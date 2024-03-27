// use alloc::string::ToString;
use log::*;
use core::sync::atomic::{AtomicUsize, Ordering};
use loongarch::consts::MAX_CORE_NUM;
use loongarch::ipi::{csr_mail_send, send_ipi_single};

pub static ENTERED_CPUS: AtomicUsize = AtomicUsize::new(1);

const TASK_STACK_SIZE: usize = 4096 * 16;

#[link_section = ".bss.stack"]
static mut SECONDARY_BOOT_STACK: [[u8; TASK_STACK_SIZE]; MAX_CORE_NUM - 1] = [[0; TASK_STACK_SIZE]; MAX_CORE_NUM - 1];
pub static mut SMP_BOOT_STACK_TOP: usize = 0;

pub static CORE_NUM: usize = MAX_CORE_NUM;

pub fn start_secondary_cpus(primary_cpu_id: usize) {
    let mut logic_cpu_id = 0;
    for i in 0..CORE_NUM {
        if i != primary_cpu_id {
            debug!("[kernel] Starting CPU {}...", i);
            unsafe { SMP_BOOT_STACK_TOP = SECONDARY_BOOT_STACK[logic_cpu_id].as_ptr() as usize + TASK_STACK_SIZE; }
            start_secondary_cpu(i);
            logic_cpu_id += 1;

            while ENTERED_CPUS.load(Ordering::Acquire) <= logic_cpu_id {
                core::hint::spin_loop();
            }
        }
    }
}

/// Starts the given secondary CPU with its boot stack.
pub fn start_secondary_cpu(hartid: usize) {
    extern "C" {
        fn _start_secondary();
    }
    csr_mail_send(_start_secondary as u64, hartid, 0);
    send_ipi_single(hartid, 1);
}

#[naked]
#[no_mangle]
#[link_section = ".text.boot"]
unsafe extern "C" fn _start_secondary() -> ! {
    core::arch::asm!("
            la.abs       $t0, {sm_boot_stack_top}
            ld.d         $sp, $t0,0          # read boot stack top

            csrrd $a0, 0x20                  # cpuid
            la.global $t0, {entry}
            jirl $zero,$t0,0
    ",
        sm_boot_stack_top = sym SMP_BOOT_STACK_TOP,
        entry = sym super::rust_main_secondary,
        options(noreturn),
    )
}