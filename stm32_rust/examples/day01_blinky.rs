#![no_std]
#![no_main]

use core::arch::asm;
use core::ptr;
use stm32_rust::gpio::GpioC;
use stm32_rust::rcc::Rcc;
use stm32_rust::time::Delay;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe { asm!("bkpt #0") }
    }
}

#[no_mangle]
#[link_section = ".vector_table.reset_vector"]
pub static RST_VECTOR: unsafe extern "C" fn() -> ! = ResetHandler;

#[no_mangle]
pub unsafe extern "C" fn ResetHandler() -> ! {
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;
    }

    let bss_start = &raw mut _sbss;
    let bss_end = &raw mut _ebss;
    let bss_len = bss_end.offset_from(bss_start) as usize;

    if bss_len > 0 {
        ptr::write_bytes(bss_start, 0, bss_len);
    }

    main()
}

fn main() -> ! {
    let rcc = Rcc::new();
    let gpioc = GpioC::new();
    let led = gpioc.pc13().into_push_pull_output(&rcc, &gpioc);
    let delay = Delay::new();

    loop {
        led.set_high();
        delay.cycles(500_000);

        led.set_low();
        delay.cycles(500_000);
    }
}
