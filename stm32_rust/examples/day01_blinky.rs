// examples/day01_blinky.rs
// Day 1: 基础寄存器操作 - 点灯
// 目标：验证环境配置正确，能够点亮 PC13 LED

#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32_rust::entry; // 从我们的库导出 (实际是 cortex_m_rt 的)

// 引入 PAC 来操作底层寄存器
use stm32f1::pac;

#[entry]
fn main() -> ! {
    // 1. 获取外设句柄
    let p = pac::Peripherals::take().unwrap();
    
    // 2. 配置 RCC (复位与时钟控制)
    // GPIOC 挂载在 APB2 总线上
    // 我们需要使能 IOPCEN 位 (位 4)
    // 0x40021018 是 RCC_APB2ENR 的地址，但在 PAC 中我们直接操作字段
    p.RCC.apb2enr.modify(|_, w| w.iopcen().set_bit());

    // 3. 配置 GPIOC PC13 为输出模式
    // PC13 属于高 8 位引脚，使用 CRH 寄存器
    // CNF13 = 00 (通用推挽), MODE13 = 01 (输出，最大 10MHz)
    // 在 PAC 中，我们可以直接写值，或者使用生成的 API
    // 这里我们直接操作位，模拟 HAL 的底层行为
    
    // CRH 寄存器偏移
    // CNF13 位: 22, 23
    // MODE13 位: 20, 21
    // 清除原有配置 (0xF << 20)
    // 设置新配置: (0b0001 << 20) -> MODE=01, CNF=00
    
    let gpioc = p.GPIOC;
    
    // 清除 PC13 配置位
    gpioc.crh.modify(|_, w| w.cnf13().bits(0b00)); 
    gpioc.crh.modify(|_, w| w.mode13().bits(0b01));

    // 4. 主循环：翻转 LED
    loop {
        // ODR (Output Data Register) 位 13 控制 PC13
        gpioc.odr.modify(|_, w| w.odr13().set_bit());
        busy_delay(500_000);
        
        gpioc.odr.modify(|_, w| w.odr13().clear_bit());
        busy_delay(500_000);
    }
}

// 简单延时函数
fn busy_delay(count: u32) {
    let mut i = 0;
    while i < count {
        // 防止编译器优化掉循环
        cortex_m::asm::nop();
        i += 1;
    }
}
