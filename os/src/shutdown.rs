pub fn shutdown(failure: bool) -> ! {
    if !failure {
        println!("Shutdown, NoReason");
    } else {
        println!("Shutdown, SystemFailure");
    }
    loop {}
    // unreachable!()
}