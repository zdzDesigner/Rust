use core::cell::UnsafeCell;
use core::ptr::{read_volatile, write_volatile};

/// [输入]：寄存器块结构体中的单个字段类型，例如 `VolatileCell<u32>`。
/// [输出]：提供带 volatile 语义的 `get` / `set` 访问接口。
/// [定位]：最小 MMIO 访问封装，给 `rcc.rs`、`gpio.rs` 等 HAL 模块使用。
/// [同步]：更新这里时，要同步检查 `src/rcc.rs`、`src/gpio.rs`、`courses/core/pointers-and-mmio.md`。
///
/// `VolatileCell<T>` 用来表达“这块内存不是普通 RAM，而是内存映射寄存器”。
///
/// 它解决两个独立问题：
///
/// 1. `UnsafeCell<T>`：打破 `&T` 的普通不可变假设，允许通过共享引用访问底层可变内存。
/// 2. `read_volatile` / `write_volatile`：告诉编译器，这些读写必须真实发生，不能被优化掉。
///
/// 这两者缺一不可：
///
/// 1. 只有 `UnsafeCell<T>`，没有 volatile 访问，编译器仍可能把寄存器读写当普通内存优化掉。
/// 2. 只有 volatile 访问，没有 `UnsafeCell<T>`，又无法正确表达“通过共享引用访问可变寄存器”这件事。
///
/// 这里的 `T` 要求是寄存器值这种可按位复制的类型，因此后面的实现限定了 `T: Copy`。
/// 当前项目里最常见的是 `u32`。
#[repr(transparent)]
pub struct VolatileCell<T> {
    /// 真正保存寄存器值视图的底层存储。
    ///
    /// 注意：这里并不是说“Rust 在本地维护了一份寄存器缓存”。
    /// 对于 MMIO 场景，`value` 最终会落在某个固定硬件地址上，
    /// 后续 `get` / `set` 都是通过该地址进行 volatile 访问。
    value: UnsafeCell<T>,
}

impl<T: Copy> VolatileCell<T> {
    /// 读取当前寄存器值。
    ///
    /// 这里使用 `read_volatile`，而不是普通解引用读取，原因是：
    ///
    /// 1. 普通读取可能被编译器重排、消除或合并。
    /// 2. 寄存器读取通常带有硬件语义，例如读取状态、清除标志、观察外设变化。
    /// 3. 对 MMIO 而言，“这次读取发生过”本身就是语义的一部分。
    ///
    /// `self.value.get()` 返回的是 `*mut T`，这里再转成 `*const T`，因为当前只是读。
    #[inline(always)]
    pub fn get(&self) -> T {
        unsafe { read_volatile(self.value.get() as *const T) }
    }

    /// 写入新的寄存器值。
    ///
    /// 这里使用 `write_volatile`，确保这次写入不会被编译器优化掉。
    /// 这对于 GPIO、RCC、串口、定时器等所有 MMIO 外设都成立。
    ///
    /// 之所以可以在 `&self` 上完成写入，是因为底层包着 `UnsafeCell<T>`：
    /// 它明确告诉编译器，这种“共享引用下的底层可变性”在这里是合法模型。
    ///
    /// 但要注意：
    ///
    /// 1. 这个类型只保证 volatile 访问语义。
    /// 2. 它不自动提供并发同步。
    /// 3. 它不自动保证读改写操作的原子性。
    ///
    /// 所以如果后面出现“主循环 + 中断”共享访问，仍需要结合临界区、原子类型
    /// 或更高层同步手段，而不能把 `VolatileCell<T>` 当成同步原语。
    #[inline(always)]
    pub fn set(&self, value: T) {
        // `UnsafeCell::get()` 返回底层存储的可变裸指针。
        // 这里不是在修改普通 Rust 变量，而是在对某个寄存器地址执行 MMIO 写入。
        unsafe { write_volatile(self.value.get(), value) }
    }
}
