#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use loongarch::register::prmd::{self, PLV};

#[no_mangle]
fn main() -> i32 {
    println!("Try to access privileged CSR in PLV3 Mode");
    println!("Kernel should kill this application!");
    unsafe {
        prmd::read().set_pplv(PLV::PLV3);
    }
    0
}
