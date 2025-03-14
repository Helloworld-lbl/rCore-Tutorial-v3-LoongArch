#![allow(unused)]

use core::ptr;

const UART_BASE: usize = 0x1fe001e0;
const UART0_THR: usize = UART_BASE + 0;
const UART0_LSR: usize = UART_BASE + 5;
const LSR_TX_IDLE: u8 = 1 << 5;

fn io_readb(addr: usize) -> u8 {
    unsafe { ptr::read_volatile(addr as *const u8) }
}

fn io_writeb(addr: usize, c: u8) {
    unsafe { ptr::write_volatile(addr as *mut u8, c) };
}

pub fn console_putchar(c: usize) {
    let mut lsr = io_readb(UART0_LSR);
    while (lsr & LSR_TX_IDLE) == 0 {
        lsr = io_readb(UART0_LSR);
    }

    io_writeb(UART0_THR, c as u8);
}