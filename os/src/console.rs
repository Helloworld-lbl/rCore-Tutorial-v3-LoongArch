//! SBI console driver, for text output

use crate::uart::console_putchar;
use core::{
    arch::asm,
    fmt::{self, Write},
};

struct Stdout;
#[link_section = ".bss.mutex"]
static MUTEX: usize = 0;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

fn acquire() {
    let mut mutex: usize = 1;
    while mutex == 1 {
        unsafe {
            asm!(
                "amswap.d $t0, $t1, $t2",
                out("$t0") mutex,
                in("$t1") 1,
                in("$t2") &MUTEX,
            )
        };
    }
}

fn release() {
    unsafe {
        asm!(
            "amswap.d $r0, $t1, $t2",
            in("$t1") 0,
            in("$t2") &MUTEX,
        )
    };
}

pub fn print(args: fmt::Arguments) {
    acquire();
    Stdout.write_fmt(args).unwrap();
    release();
}

/// print string macro
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

/// println string macro
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
