#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Into Test store_fault, we will insert an invalid store operation...");
    println!("Kernel should kill this application!");
    unsafe {
        let rom_ptr:usize = 0xffffffffffffffff;
        (rom_ptr as *mut u8).write_volatile(0);
        // core::ptr::null_mut::<u8>().write_volatile(0);
    }
    0
}
