mod ahci;
pub mod pci;

use crate::drivers::pci::pci_init;
pub use ahci::AHCIDriver;
use alloc::sync::Arc;
use core::cell::UnsafeCell;
use core::mem::transmute;
use core::ops::{Deref, DerefMut};
use easy_fs::BlockDevice;
use log::info;
use lazy_static::*;

// use crate::board::BlockDeviceImpl;

lazy_static! {
    pub static ref BLOCK_DEVICE: Arc<dyn BlockDevice> = Arc::new(pci_init().unwrap());
}

#[allow(unused)]
pub fn block_device_test() {
    println!("Block device test...");
    let block_device = BLOCK_DEVICE.clone();
    let mut write_buffer = [0u8; 512];
    let mut read_buffer = [0u8; 512];
    for i in 0..512 {
        for byte in write_buffer.iter_mut() {
            *byte = i as u8;
        }
        block_device.write_block(i as usize, &write_buffer);
        block_device.read_block(i as usize, &mut read_buffer);
        assert_eq!(write_buffer, read_buffer);
    }
    println!("Block device test passed!");
}

#[derive(Debug, Default)]
#[repr(transparent)]
pub struct Cell<T>(UnsafeCell<T>);

unsafe impl<T> Sync for Cell<T> {}

impl<T> Cell<T> {
    #[inline(always)]
    pub fn get(&self) -> &mut T {
        unsafe { &mut *self.0.get() }
    }
}

impl<T> Deref for Cell<T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> DerefMut for Cell<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get()
    }
}
