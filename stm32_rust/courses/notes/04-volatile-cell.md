# 04 - VolatileCell 与 UnsafeCell (0 依赖最佳实践)

在"0 依赖”的 Rust 嵌入式项目中，如何优雅且安全地处理硬件寄存器的 Volatile 读写是一个核心工程问题。

---

## 1. 原始写法的痛点

如果我们直接使用 `core::ptr::read_volatile`，代码会变得极其繁琐且不可读：

```rust
// 痛点：满屏的 unsafe 和 ptr 操作，掩盖了业务逻辑
let val = unsafe { core::ptr::read_volatile(addr) };
unsafe { core::ptr::write_volatile(addr, val | 0x01) };
```

这导致代码难以维护，尤其是当寄存器操作非常频繁时。

---

## 2. 为什么需要 `UnsafeCell`？（核心难点）

你可能会问：*“为什么不能直接把寄存器结构体定义为 `&mut RCC_Regs`，然后直接读写？”*

**答案：因为别名（Aliasing）。**

在嵌入式系统中，中断（ISR）和主循环（Main Loop）可能会同时访问同一个寄存器。
*   Rust 的规则是：`&mut T` 必须是**唯一**的访问路径（Exclusive Access）。
*   如果你在 `main` 中持有了 `&mut RCC_Regs`，而中断服务程序也试图访问它，这就违反了 Rust 的借用规则（UB，未定义行为）。

**解决方案**：我们需要一种类型，它允许通过**共享引用（`&self`）** 来进行**可变访问**。这就是 `UnsafeCell<T>` 的作用。它是 Rust 中所有“内部可变性”（Interior Mutability）的基石。

---

## 3. 解决方案：VolatileCell 模式

我们在 `src/volatile.rs` 中实现了一个结合了 `Volatile` 和 `UnsafeCell` 的封装。

### 核心原理
1.  **`UnsafeCell<T>`**：告诉编译器“这块内存可以通过共享引用被修改（例如被中断修改）”。
2.  **`#[repr(transparent)]`**：保证 `VolatileCell<u32>` 在内存中与 `u32` 完全等价。
3.  **Safe API**：对外暴露 `get()` 和 `set()` 方法，内部处理 unsafe 操作。

### 代码示例
```rust
use core::cell::UnsafeCell;

#[repr(transparent)]
pub struct VolatileCell<T> {
    value: UnsafeCell<T>, // 允许通过 &self 修改
}

impl<T: Copy> VolatileCell<T> {
    // 读取
    pub fn get(&self) -> T {
        // self.value.get() 返回 *mut T，我们转为 *const T 读取
        unsafe { core::ptr::read_volatile(self.value.get() as *const T) }
    }

    // 写入
    pub fn set(&self, value: T) {
        // self.value.get() 返回 *mut T，直接写入
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }
}
```

### 优势
*   **安全性**：既防止了编译器优化（Volatile），又符合 Rust 的别名规则（UnsafeCell）。
*   **可读性**：`rcc.apb2enr.set(...)` 非常直观。
*   **零成本**：`inline(always)` 确保编译器将其内联，生成的汇编代码与手写一致。

---

## 4. 项目实战

在 `stm32_rust` 项目中，我们使用了 `VolatileCell` 来定义所有寄存器字段：

```rust
// src/main.rs
let rcc = &*RCC_BASE; // 注意：这里我们获取的是共享引用 &RCC_Regs
// 但依然可以安全地写入，因为 VolatileCell 处理了内部可变性
rcc.apb2enr.set(0x01); 
```
这种设计是 Rust 嵌入式 HAL 库（如 `stm32f1xx-hal`）处理内存映射 I/O 的标准范式。

*注：本笔记记录了 0 依赖项目中硬件访问的高级封装方法，特别是解决了中断环境下的别名问题。*
