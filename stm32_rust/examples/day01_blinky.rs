#![no_std]
#![no_main]

use core::arch::asm;
use core::ptr;
use stm32_rust::gpio::GpioB;
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
        // `.data` 的加载地址在 Flash，运行地址在 RAM。
        // 启动时必须先把 Flash 中的初始值拷贝到 RAM，否则带初始值的静态变量会读到错误数据。
        static mut _sidata: u8;
        static mut _sdata: u8;
        static mut _edata: u8;

        // `.bss` 只在 RAM 中占位，启动时需要清零。
        static mut _sbss: u8;
        static mut _ebss: u8;
    }

    // 初始化 `.data`：Flash(_sidata) -> RAM(_sdata.._edata)。
    // 使用 raw reference，避免对 linker symbol 创建普通 Rust 引用。
    let data_src = &raw const _sidata;
    let data_start = &raw mut _sdata;
    let data_end = &raw mut _edata;
    let data_len = data_end.offset_from(data_start) as usize;

    if data_len > 0 {
        // Flash 和 RAM 地址区间不会重叠，使用 nonoverlapping copy。
        ptr::copy_nonoverlapping(data_src, data_start, data_len);
    }

    // 初始化 `.bss`：RAM(_sbss.._ebss) 全部置零。
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
    let gpiob = GpioB::new();
    let led = gpiob.pb12().into_push_pull_output(&rcc, &gpiob);
    let delay = Delay::new();

    loop {
        led.set_high();
        delay.cycles(500_00);

        led.set_low();
        delay.cycles(500_00);
    }
}
