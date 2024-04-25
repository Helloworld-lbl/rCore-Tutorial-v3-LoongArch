use core::{arch::asm, ptr};
use loongarch;

#[allow(unused)]
/// use sbi call to shutdown the kernel
pub fn shutdown(failure: bool) -> ! {
    unsafe {
        asm!(
            "st.d {}, {}, 0",
            in(reg) 0xff,
            in(reg) 0x10080010,
        );
        // unsafe { ptr::write_volatile(0x10080010 as *mut u8, 0x00) };
        // let mut a = loongarch::register::crmd::read();
        // a.set_ie(false);
        // a.write();
        // asm!("idle 0");
    }
    unreachable!()
}
