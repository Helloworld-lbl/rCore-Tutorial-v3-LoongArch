#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use loongarch::register::eentry::{self};

#[no_mangle]
fn main() -> i32 {
    println!("Try to access privileged CSR in PLV3 Mode");
    println!("Kernel should kill this application!");
    unsafe {
        eentry::write(0 as usize);
    }
    0
}
