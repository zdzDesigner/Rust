# Rust `core` 库学习路线图与架构解析指南

本文档提供了一套基于 **“自底向上、抓大放小”** 策略的 `core` 库学习路径。针对已有 Go/Zig 经验及 STM32 嵌入式开发背景的开发者，旨在高效掌握 Rust 底层内存模型、Trait 协议与编译器桥接机制。

---

## 🎯 核心学习原则
1. **拒绝按文件顺序线性阅读**：`core` 内部充满交叉引用、宏展开与编译器内部标记，线性阅读极易迷失。
2. **抓住“最小集”本质**：`core` 是语言特性的基础设施，**不包含堆分配（`alloc`）与 OS 调用（`std`）**。所有现代语法（`for`, `?`, `+`）最终都会降级为对 `core` 中 Trait 的调用。
3. **结合嵌入式场景**：重点关注指针操作、内存布局、原子指令与零成本抽象，忽略与文件系统/网络相关的高级封装。

---

## 📍 第一阶段：内存与基石（原子操作层）
**目标**：理解 Rust 如何安全地操纵裸内存。这是手写 HAL 库与 `no_std` 开发的核心。

### 1. `src/marker.rs` (编译器标记与类型系统)
* **重点内容**：`Send`, `Sync`, `Sized`, `PhantomData`, `Unpin`
* **阅读策略**：跳过空实现，专注文档注释。区分 `Send` / `Sync` / `Unpin` 这类 Auto Traits、`Sized` 这类编译器特殊 Trait，以及 `PhantomData` 这种零大小类型工具。
* **核心考点**：为什么 `*mut T` 默认不是 `Send`？`PhantomData` 如何在不占用内存的情况下影响生命周期检查？
* 🔗 **关联阅读**：[派生宏与自动 Trait 机制](./trait-bounds-derive-macros.md)

### 2. `src/mem/mod.rs` (内存操作原语)
* **重点内容**：`size_of`, `align_of`, `transmute`, `swap`, `replace`, `MaybeUninit`
* **核心考点**：`MaybeUninit` 是在 Rust 中表达 **“这块内存当前可能未初始化”** 的标准且推荐的安全抽象（可对照 Zig 的 `undefined` 来理解）。`transmute` 的严格对齐与类型大小要求。
* 🛠️ **嵌入式场景**：DMA 缓冲区初始化、寄存器位域强制转换、避免零初始化带来的性能损耗。

### 3. `src/ptr/mod.rs` (裸指针安全封装)
* **重点内容**：`read_volatile`, `write_volatile`, `copy_nonoverlapping`, `addr_of!`
* **核心考点**：`addr_of!` 与 `&raw` 如何避免先创建非法引用；如果目标字段可能未对齐，后续访问通常还要配合 `read_unaligned` / `write_unaligned` 等 API。
* 🛠️ **嵌入式场景**：你之前实现的 `VolatileCell` 底层调用的正是这里的 API。对比标准库的 `ptr::read_volatile` 理解 MMIO 读写的安全边界。

---

## 📍 第二阶段：契约与语法糖（Trait 协议层）
**目标**：理解 Rust 语法背后的 Trait 协议。Rust 的许多“关键字”本质上是 Trait 的语法糖。

### 4. `src/ops/mod.rs` (运算符重载与解引用)
* **重点内容**：`Add`, `Sub`, `Deref`, `DerefMut`, `Index`, `IndexMut`, `Drop`
* **核心考点**：`Deref` 是智能指针（`Box`, `&T`）自动解引用的魔法所在。`Drop` 的确定性调用时机。
* 💡 **认知突破**：`Deref` 主要参与智能指针的人体工学，例如方法解析与自动借用/自动解引用；而一元 `*` 解引用本身还涉及语言内建的引用/指针解引用规则，不能简单等同于“展开成 `.deref()`”。

### 5. `src/iter/mod.rs` (迭代器协议)
* **重点内容**：`Iterator`, `IntoIterator`, `FromIterator`
* **核心考点**：理解 `next() -> Option<Self::Item>` 的单一方法设计。**惰性求值**与编译期内联展开。
* 🛠️ **嵌入式场景**：处理传感器数据流、日志缓冲时，迭代器链（`.filter().map()`) 在 Release 模式下会被优化为等效的 `while` 循环，零额外开销。

### 6. `src/option.rs` & `src/result.rs` (错误与空值处理)
* **重点内容**：`unwrap`, `expect`, `map`, `and_then`, `ok_or`
* **核心考点**：`#[must_use]` 属性的强制检查机制。`?` 操作符底层对 `Try` trait 的展开逻辑。

---

## 📍 第三阶段：并发与内部可变性（安全机制层）
**目标**：掌握 Rust 如何在不依赖 GC 的情况下实现内存安全与并发安全。

### 7. `src/cell.rs` (内部可变性体系)
* **重点内容**：`Cell<T>` (值拷贝), `RefCell<T>` (运行时检查), `UnsafeCell<T>` (编译器后门)
* **核心考点**：`UnsafeCell` 是标准库中 **唯一** 能打破 `&T` 不可变假设的基础类型。它是所有并发原语（Mutex, RwLock）与内部可变性封装的基石。
* 🛠️ **嵌入式场景**：`Cell` 适合单线程/单上下文里的轻量内部可变状态；如果涉及中断与主循环共享，通常还要结合临界区、`Mutex` 或其他同步封装；复杂数据结构借用检查选 `RefCell`（有 Panic 风险）；底层驱动封装常会落到 `UnsafeCell`。
* 🔗 **关联阅读**：[Cell 与内部可变性详解](../../../../Try/Blog/Rust/10-rust-cell-interior-mutability.md)

### 8. `src/sync/atomic.rs` (硬件级原子操作)
* **重点内容**：`AtomicBool`, `AtomicU32`, `AtomicPtr`, `Ordering`
* **核心考点**：`Acquire`, `Release`, `SeqCst` 内存屏障语义。这些操作由编译器映射到目标平台支持的原子指令或等价机制，**不依赖 OS**，但是否支持某种原子宽度仍取决于 target/CPU 能力。
* 💡 **架构意义**：正因为原子操作是硬件指令，Rust 才能将它们放在 `core` 中，使得 `no_std` 环境也能实现无锁数据结构。

---

## 📍 第四阶段：编译器桥梁（Intrinsics 与架构指令）
**目标**：窥探编译器魔法。这部分代码没有函数体，由 LLVM 直接提供实现。

### 9. `src/intrinsics/mod.rs` (LLVM 内部函数)
* **重点内容**：`volatile_load`, `atomic_store`, `size_of_val`, `transmute`
* **阅读策略**：只看函数签名与 `#[rustc_intrinsic]` 标记。理解它们是编译器与 Rust 的 **“私有协议”**。
* ⚠️ **警告**：这些函数极不稳定，普通开发者应通过 `core::mem` 或 `core::ptr` 的公开 API 间接调用，**不要直接使用**。

### 10. `src/arch/` (架构特定指令)
* **重点内容**：`x86`, `arm`, `riscv` 等子模块。
* **核心考点**：`asm!` 宏与架构专属指令（如 ARM 的 `wfi`, `dsb`，x86 的 `rdtsc`）。
* 🛠️ **嵌入式场景**：低功耗休眠、内存屏障、性能计数器读取的最终落脚点。

---

## 🛠️ 学习策略与避坑指南

| 误区 | 正确做法 |
| :--- | :--- |
| 试图读懂所有宏 (`internal_macros.rs`) | 跳过宏定义。使用 `cargo expand` 查看宏展开后的真实代码，更易理解。 |
| 线性阅读，遇到不懂的交叉引用就卡住 | **黑盒思维**：先接受 API 契约，跑通示例代码，再深入底层实现。 |
| 混淆 `core` 与 `std` 的边界 | 牢记：`core` = 无 OS/无堆；`alloc` = 无 OS/有堆；`std` = 有 OS。`core::mem` 和 `std::mem` 是同一个东西，`std` 只是重新导出了 `core`。 |
| 忽略文档注释只看代码 | `core` 的源码注释质量极高，包含大量设计决策（RFC 引用）和 UB 警告。**文档即规范**。 |

---

## 🧪 实战思考题（检验学习成果）

1. **分配边界**：为什么 `String` 和 `Vec<T>` 不能放在 `core` 里，而 `&str` 和 `&[T]` 可以？
2. **零成本验证**：`Option<&T>` 在内存布局上为什么和裸指针 `*const T` 大小完全一样？（提示：Niche Optimization / 空指针优化）
3. **安全契约**：`UnsafeCell` 允许别名修改，但为什么标准库要求开发者自己保证数据竞争安全？编译器在这里放弃了什么权利？
4. **跨语言对照**：Rust 的 `MaybeUninit<T>` 与 Zig 的 `undefined` / `std.mem.zeroes(T)` 在处理未初始化内存时，各自的心智模型差异是什么？

*注：本文档作为 `01-RUST库体系概述.md` 的配套实践指南，建议结合 `rust-lang/rust` 仓库源码与 `cargo doc` 同步阅读。*
