#![no_std]
#![no_main]

// 引入我们自定义的 Volatile 模块
mod volatile;
use volatile::VolatileCell;

use core::arch::asm;
use core::ptr;

// --- 1. Panic Handler (0 依赖标准写法) ---
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        // 触发 HardFault 或断点，方便调试器捕获
        unsafe { asm!("bkpt #0") }
    }
}

// --- 2. 寄存器结构体定义 (使用 VolatileCell 确保安全访问) ---
// 使用 VolatileCell 替代普通 u32，防止编译器优化掉硬件读写操作
#[repr(C)]
struct RCC_Regs {
    _cr: VolatileCell<u32>,
    _cfgr: VolatileCell<u32>,
    _cir: VolatileCell<u32>,
    _apb2rstr: VolatileCell<u32>,
    _apb1rstr: VolatileCell<u32>,
    _ahbenr: VolatileCell<u32>,
    apb2enr: VolatileCell<u32>, // 关键：使用 VolatileCell
    _apb1enr: VolatileCell<u32>,
    _bdcr: VolatileCell<u32>,
    _csr: VolatileCell<u32>,
}

#[repr(C)]
struct GPIOC_Regs {
    _crl: VolatileCell<u32>,
    crh: VolatileCell<u32>, // PC13 在 CRH
    _idr: VolatileCell<u32>,
    _odr: VolatileCell<u32>,
    bsrr: VolatileCell<u32>,
    _brr: VolatileCell<u32>,
    _lckr: VolatileCell<u32>,
}

// 基地址映射
const RCC_BASE: *const RCC_Regs = 0x4002_1000 as _;
const GPIOC_BASE: *const GPIOC_Regs = 0x4001_1000 as _;

// --- 3. 向量表注入 ---
// 必须放在 .vector_table.reset_vector 段
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RST_VECTOR: unsafe extern "C" fn() -> ! = ResetHandler;

// --- 4. ResetHandler (启动入口) ---
// 负责初始化 BSS/Data 并跳转到 main
#[no_mangle]
pub unsafe extern "C" fn ResetHandler() -> ! {
    // 声明链接脚本中的符号
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;
    }

    // 1. 清零 .bss 段
    // 计算长度。这里直接创建原始指针，避免对 mutable static 创建引用。
    let bss_start = &raw mut _sbss;
    let bss_end = &raw mut _ebss;
    let bss_len = bss_end.offset_from(bss_start) as usize;

    if bss_len > 0 {
        ptr::write_bytes(bss_start, 0, bss_len);
    }

    // 注意：.data 段的拷贝在这里通常也是需要的，但为了保持示例最小化，
    // 且当前代码没有使用已初始化的静态变量，我们暂时跳过 .data 拷贝。
    // 完整的实现应包含：ptr::copy_nonoverlapping(&_sidata, &mut _sdata, data_len);

    // 2. 跳转到主函数
    main()
}

// --- 5. 主程序 ---
fn main() -> ! {
    // 获取寄存器指针
    // 解引用裸指针是 unsafe 操作
    let (rcc, gpioc) = unsafe { (&*RCC_BASE, &*GPIOC_BASE) };

    // 1. 使能 GPIOC 时钟 (IOPCEN = Bit 4)
    // 使用 VolatileCell 的 update 进行 Read-Modify-Write。
    rcc.apb2enr.update(|value| value | (1 << 4));

    // 内存屏障：确保时钟开启后再操作 GPIO
    unsafe { asm!("dsb", "isb") };

    // 2. 配置 PC13 为推挽输出 2MHz
    // PC13 在 CRH (Pin 13 is index 5 in high nibble)
    // CNF=00, MODE=01 -> 0b0001
    // 位移计算: (13 - 8) * 4 = 20
    let shift = 20;
    let mode_bits = 0b0001;

    // 清除旧配置，写入新配置
    gpioc
        .crh
        .update(|value| (value & !(0xF << shift)) | (mode_bits << shift));

    // 3. 主循环
    loop {
        // 点亮 LED (BSRR 低 16 位有效)
        gpioc.bsrr.write(1 << 13);
        busy_delay();

        // 熄灭 LED (BSRR 高 16 位有效)
        gpioc.bsrr.write(1 << (13 + 16));
        busy_delay();
    }
}

/// 鲁棒的延时函数，防止被编译器优化掉
fn busy_delay() {
    let mut i: u32 = 0;
    while i < 500_000 {
        // black_box 告诉编译器 'i' 有副作用，防止循环被优化删除
        core::hint::black_box(i);
        i += 1;
    }
}
