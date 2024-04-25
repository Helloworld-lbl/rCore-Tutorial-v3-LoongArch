//! Implementation of [`TrapContext`]

use loongarch::register::prmd::{self, Prmd, PLV};

#[repr(C)]
#[derive(Debug)]
/// trap context structure containing sstatus, sepc and registers
pub struct TrapContext {
    /// general regs[0..31]
    pub r: [usize; 32],
    /// CSR prmd      
    pub prmd: Prmd,
    /// CSR era
    pub era: usize,
    /// Addr of Page Table
    pub kernel_pgdl: usize,
    /// Addr of trap_handler function
    pub trap_handler: usize,
}

impl TrapContext {
    /// set stack pointer to r_3 reg (sp)
    pub fn set_sp(&mut self, sp: usize) {
        self.r[3] = sp;
    }
    /// init app context
    pub fn app_init_context(
        entry: usize,
        sp: usize,
        kernel_pgdl: usize,
        trap_handler: usize,
    ) -> Self {
        let mut prmd = prmd::read(); // CSR sstatus
        prmd.set_pplv(PLV::PLV3); //previous privilege mode: plv3
        let mut cx = Self {
            r: [0; 32],
            prmd,
            era: entry, // entry point of app
            kernel_pgdl, // addr of page table
            trap_handler, // addr of trap_handler function
        };
        cx.set_sp(sp); // app's user stack pointer
        cx // return initial Trap Context of app
    }
    pub fn from_existed(existed: &TrapContext) -> TrapContext {
        Self {
            r: existed.r,
            prmd: existed.prmd,
            era: existed.era,
            kernel_pgdl: existed.kernel_pgdl,
            trap_handler: existed.trap_handler,
        }
    }
}
