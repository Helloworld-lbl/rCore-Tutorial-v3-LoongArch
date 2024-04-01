/// use sbi call to shutdown the kernel
pub fn shutdown(failure: bool) -> ! {
    // use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    // if !failure {
    //     system_reset(Shutdown, NoReason);
    // } else {
    //     system_reset(Shutdown, SystemFailure);
    // }
    loop {}
    unreachable!()
}