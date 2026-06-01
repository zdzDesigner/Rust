# 03 - 硬件寄存器访问与 Volatile 语义

在嵌入式开发中，读写内存映射寄存器（MMIO）与读写普通 RAM 有本质区别。编译器往往会尝试优化掉“看似无用”的读或写，这在硬件操作中会导致致命错误。

---

## 1. 为什么需要 `volatile`？

假设你向一个硬件寄存器连续写入相同的值。
*   **普通 RAM**：编译器可能认为第二次写入是多余的，直接**优化删除**。
*   **硬件寄存器**：寄存器访问可能有副作用。写同一个值也可能清中断标志、启动外设操作或推进 FIFO；读操作也可能清状态位。因此每次访问都必须真实发生。

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

#### ❌ 错误写法（普通引用不适合 MMIO）
```rust
let ptr = 0x4002_1018 as *mut u32;
let reg_ref = unsafe { &mut *ptr };
*reg_ref |= 0x01;
```

这不仅可能被优化，还会把 MMIO 地址伪装成普通 Rust 可变引用。`&mut T` 隐含唯一访问假设，而硬件寄存器可能被外设、中断或硬件本身异步修改。MMIO 应使用裸指针配合 `read_volatile/write_volatile`，或封装成 `VolatileCell`。

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

除了 `volatile`，某些底层场景还需要**内存屏障**。例如修改系统控制寄存器、切换中断状态、进入低功耗，或需要确保外设写入在后续操作前完成时，可能需要 `DSB`/`ISB`。这和 volatile 解决的问题不同。

*   **Zig**：严格同步时可使用 atomic fence，或在目标架构需要时插入 `dsb`/`isb` 等指令。
*   **Rust**：可使用 `core::sync::atomic::compiler_fence` 限制编译器重排；需要真实 CPU 屏障时使用 `core::arch::asm!("dsb", "isb")` 等目标相关指令。

---

## 5. 对 STM32 项目的建议

在你的 `stm32_rust` (0 依赖) 项目中，最直接、最严谨的方式是使用 `read_volatile/write_volatile`：

```rust
use core::ptr::{read_volatile, write_volatile};

let apb2enr = 0x4002_1018 as *mut u32;
unsafe {
    let value = read_volatile(apb2enr);
    write_volatile(apb2enr, value | (1 << 4));
}
```

但直接散落 `read_volatile/write_volatile` 会显著降低可读性。实际工程中通常封装一个 `VolatileCell<T>` 或寄存器 wrapper，让 MMIO 字段通过安全方法完成 volatile 访问。

*注：本笔记记录了底层硬件访问的安全性问题，是 0 依赖开发的基石。*
