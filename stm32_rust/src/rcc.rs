use crate::volatile::VolatileCell;
use core::arch::asm;

// 1. 核心结论：它是“内部可变性”的唯一根基
// UnsafeCell<T> 是 Rust 标准库中唯一允许打破“不可变引用不能修改数据”这一铁律的类型。
//
// 在 Rust 中：
// *   &T (不可变引用)：编译器保证这块内存只读。
// *   &mut T (可变引用)：编译器保证这块内存是独占的。
//
// 问题：如果你想让多个 &T 共享一块数据，同时又允许通过其中一个 &T 修改它，怎么办？
// 答案：用 UnsafeCell<T> 包裹它。它是所有“内部可变性”类型（Mutex, RwLock, RefCell, Cell, AtomicU32）的底层实现基础。
// 2. 底层机制：为什么需要它？
// 这不是为了方便，而是为了 防止 LLVM 优化掉你的代码。
// LLVM 的激进优化
// LLVM 假设：如果你拿到了一个 &T，只要你不通过 unsafe 强转，这块内存就不会变。它会缓存变量的值。
//
// rust
// // 没有 UnsafeCell 的情况
// let x = &some_value;
// let a = *x;
// // ... 做一些无关操作 ...
// let b = *x;
//
// // LLVM 可能会优化为：let b = a; (因为它认为 x 指向的值没变)
//
// UnsafeCell 的作用
// 一旦你把数据放进 UnsafeCell，你就告诉编译器：“即使我有 &T，这块内存也可能在后台被修改（比如被硬件中断、被另一个线程），不要缓存它的值，每次都要去内存里读！”
//
// 它通过提供 .get() 方法，返回一个裸指针 mut T。有了裸指针，你就可以在 unsafe 块里修改数据。

#[repr(C)]
struct Regs {
    _cr: VolatileCell<u32>,
    _cfgr: VolatileCell<u32>,
    _cir: VolatileCell<u32>,
    _apb2rstr: VolatileCell<u32>,
    _apb1rstr: VolatileCell<u32>,
    _ahbenr: VolatileCell<u32>,
    apb2enr: VolatileCell<u32>,
    _apb1enr: VolatileCell<u32>,
    _bdcr: VolatileCell<u32>,
    _csr: VolatileCell<u32>,
}

const RCC_BASE: *const Regs = 0x4002_1000 as *const Regs;
const IOPBEN: u32 = 1 << 3;
const IOPCEN: u32 = 1 << 4;

pub struct Rcc {
    regs: *const Regs,
}

impl Rcc {
    pub const fn new() -> Self {
        Self { regs: RCC_BASE }
    }

    pub fn enable_gpiob(&self) {
        let regs = unsafe { &*self.regs };
        regs.apb2enr.update(|value| value | IOPBEN);

        // 使能时钟后插入屏障，避免后续 GPIO 访问过早执行。
        unsafe { asm!("dsb", "isb") };
    }

    pub fn enable_gpioc(&self) {
        let regs = unsafe { &*self.regs };
        regs.apb2enr.update(|value| value | IOPCEN);

        // 使能时钟后插入屏障，避免后续 GPIO 访问过早执行。
        unsafe { asm!("dsb", "isb") };
    }
}
