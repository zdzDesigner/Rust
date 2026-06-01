use core::cell::UnsafeCell;
use core::ptr::{read_volatile, write_volatile};

/// A wrapper for memory-mapped I/O registers.
/// Ensures that reads and writes are not optimized away by the compiler.
/// Uses `UnsafeCell` to allow mutation through a shared reference (`&self`),
/// which is the correct semantic for hardware registers (aliasing is possible via interrupts).
#[repr(transparent)]
pub struct VolatileCell<T> {
    value: UnsafeCell<T>,
}

impl<T: Copy> VolatileCell<T> {
    /// Read the current value of the register.
    #[inline(always)]
    pub fn get(&self) -> T {
        unsafe { read_volatile(self.value.get() as *const T) }
    }

    /// Write a new value to the register.
    #[inline(always)]
    pub fn set(&self, value: T) {
        // We cast &self (which holds UnsafeCell) to *mut T.
        // UnsafeCell allows mutation through shared references, making this legal for MMIO.
        unsafe { write_volatile(self.value.get(), value) }
    }
}
