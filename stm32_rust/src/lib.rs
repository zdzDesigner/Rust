#![no_std]

// 导出我们后续要编写的模块
pub mod gpio;
pub mod rcc;
pub mod time;

// 引入 cortex-m-rt 入口点
pub use cortex_m_rt::entry;
