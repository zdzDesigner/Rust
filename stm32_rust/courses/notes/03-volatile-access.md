# 03 - 硬件寄存器访问与 Volatile 语义

在嵌入式开发中，读写内存映射寄存器（MMIO）与读写普通 RAM 有本质区别。编译器往往会尝试优化掉“看似无用”的读或写，这在硬件操作中会导致致命错误。

---

## 1. 为什么需要 `volatile`？

假设你向 GPIO 数据寄存器写入 1（点亮 LED），然后紧接着再次写入 1（还是点亮 LED）。
*   **普通 RAM**：编译器会认为第二次写入是多余的，直接**优化删除**。
*   **硬件寄存器**：虽然值没变，但**电信号必须翻转或重新触发**。如果你被优化掉了，LED 可能就不亮了，或者中断标志位无法清除。

我们需要告诉编译器：**“这块内存很特殊，每次访问都必须真实发生，绝对不能优化。”**

---

## 2. Zig 的实现：`*volatile` 指针

Zig 直接在类型系统级别支持 `volatile`。当你把一个指针声明为 `*volatile` 时，编译器会自动对该指针的所有读写加上 `volatile` 属性。

### Zig 代码示例
```zig
const ADDR = 0x40021018; // RCC_APB2ENR
// 转换为 volatile 指针
const reg_ptr: *volatile u32 = @as(*volatile u32, @ptrFromInt(ADDR));

// 读写操作自动包含 volatile 语义
reg_ptr.* = 0x01; 
if (reg_ptr.* > 0) {}
```
**优势**：一旦定义为 volatile，后续所有使用该指针的代码都安全，无需重复标记。

---

## 3. Rust 的现状：显式调用

Rust 的标准引用（如 `&u32` 或 `&mut u32`）**默认是不带 volatile 语义的**。
虽然有 crate（如 `volatile-cell`）可以封装它，但在 **0 依赖** 模式下，你必须使用 `core::ptr` 提供的函数。

### Rust 代码示例 (0 依赖写法)

#### ❌ 错误写法（可能被优化）
```rust
let ptr = 0x4002_1018 as *mut u32;
let reg_ref = unsafe { &mut *ptr };
reg_ref |= 0x01; // 编译器可能会优化这次操作
```

#### ✅ 正确写法（使用 `core::ptr`）
```rust
use core::ptr::{read_volatile, write_volatile};

let addr = 0x4002_1018 as *mut u32;
unsafe {
    // 读取当前值
    let val = read_volatile(addr);
    // 写入新值
    write_volatile(addr, val | 0x01);
}
```
**缺点**：操作变得繁琐，不再是直观的运算符重载。

---

## 4. 进阶：内存屏障 (Barriers)

除了 `volatile`，现代 CPU 还有**乱序执行**的问题。你写了 A 寄存器，CPU 可能先执行 B 寄存器的写操作。

*   **Zig**：通常编译器在处理 MMIO 时表现良好，但在严格同步时需要 `std.atomic.fence(.SeqCst)` 或内联汇编 `asm volatile("dsb");`。
*   **Rust**：同样使用 `core::arch::asm!("dsb", "isb")` 或者原子操作的 fence 函数。

---

## 5. 对 STM32 项目的建议

在你的 `stm32_rust` (0 依赖) 项目中，为了保持代码简洁且不被过度优化，我们可以采用一种折衷方案：**使用结构体引用，但在关键操作处插入 Compiler Barrier（编译器屏障）。**

```rust
// 读取后立即赋值给自己，或者加一个 asm!("") 伪汇编防止优化
rcc.apb2enr |= 1 << 4;
unsafe { core::arch::asm!("nop") }; // 简单的屏障暗示
```
*当然，最严谨的方式是用 `read_volatile/write_volatile`，但这会让代码变成一坨屎山。在实际工程中，通常会在第 100 行写一个泛型 wrapper `struct Volatile<T>(T)` 来解决这个问题。*

*注：本笔记记录了底层硬件访问的安全性问题，是 0 依赖开发的基石。*
