use loongarch::register::prmd::{self, Prmd, PLV};
/// Trap Context
#[repr(C)]
pub struct TrapContext {
    /// general regs[0..31]
    pub r: [usize; 32],
    /// CSR prmd      
    pub prmd: Prmd,
    /// CSR era
    pub era: usize,
}

impl TrapContext {
    /// set stack pointer to r_3 reg (sp)
    pub fn set_sp(&mut self, sp: usize) {
        self.r[3] = sp;
    }
    /// init app context
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut prmd = prmd::read(); // CSR sstatus
        prmd.set_pplv(PLV::PLV3); //previous privilege mode: plv3
        let mut cx = Self {
            r: [0; 32],
            prmd,
            era: entry, // entry point of app
        };
        cx.set_sp(sp); // app's user stack pointer
        cx // return initial Trap Context of app
    }
}
