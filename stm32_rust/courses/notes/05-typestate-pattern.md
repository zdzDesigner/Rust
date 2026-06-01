# 05 - 类型状态机 (Typestate) 模式

这是 Rust 嵌入式 HAL 库（如 `stm32f1xx-hal`）中最核心的设计模式，也是 Rust 相比 C 和 Zig 在**安全性**上最大的优势之一。

---

## 1. 什么是类型状态？

在嵌入式开发中，引脚（Pin）是有状态的。一个引脚可以被配置为输入、输出、复用功能等。
*   **C 语言**：通常用 `enum PinMode` 记录状态，但在运行时检查。如果你在 Output 模式下调用 Read，可能会读到错误数据或导致硬件冲突，编译器无法阻止你。
*   **Zig**：可以把 Mode 作为运行时字段 `pin.mode` 并用 `switch (pin.mode)` 检查，也可以通过 `comptime` 参数在编译期建模。
*   **Rust (Typestate)**：将状态提升到**类型系统**中。
    *   一个引脚不仅仅是 `Pin`，而是 `Pin<Mode::Input>` 或 `Pin<Mode::Output>`。
    *   **编译器保证**：你无法对 `Pin<Output>` 调用 `read()` 方法，因为该方法只定义在 `Pin<Input>` 上。

---

## 2. Rust 实现示例

```rust
// 1. 定义状态类型（通常使用零大小类型 ZST）
pub struct Input;
pub struct Output;

// 2. 引脚结构体，包含一个泛型参数 M 代表 Mode
pub struct Pin<M> {
    pub(crate) pin_num: u8,
    _mode: core::marker::PhantomData<M>, // 标记状态，不占用内存
}

// 3. 只有 Input 模式的引脚才能实现 read
impl Pin<Input> {
    pub fn read(&self) -> bool {
        // 读取硬件寄存器...
        true
    }
}

// 4. 状态转换方法
// 这个函数消耗掉当前的 Pin<M>，返回一个新的 Pin<NewM>
impl<M> Pin<M> {
    pub fn into_output(self) -> Pin<Output> {
        // 配置硬件寄存器...
        Pin {
            pin_num: self.pin_num,
            _mode: core::marker::PhantomData,
        }
    }
}

// 使用：
// let p = Pin { ... }; // 假设默认是 Input
// let val = p.read();  // ✅ 编译通过
// p.read();            // ❌ 编译失败：p 已经被 move 到 into_output 了
// let out = p.into_output(); 
// out.read();          // ❌ 编译失败：Pin<Output> 没有 read 方法！
```

---

## 3. 与 Zig 的对比

| 特性 | Zig (显式状态) | Rust (Typestate) | 差异核心 |
| :--- | :--- | :--- | :--- |
| **状态表示** | `struct Pin { mode: Mode, ... }` | `struct Pin<Mode>` (泛型) | 字段 vs 类型参数 |
| **检查时机** | 可运行时检查，也可用 `comptime` 编译期建模 | **编译期** (Compile-time) | Rust Typestate 默认把状态放进类型系统 |
| **转换方式** | `pin.mode = .Output;` (修改字段) | `pin.into_output()` (所有权转移) | 修改 vs 替换 |
| **内存开销** | `mode` 字段占用空间 (通常 1 byte) | `PhantomData` 占用 0 字节 | **零成本抽象** |

### 为什么 Rust 选择 Typestate？
Rust 的所有权机制允许"消耗旧对象，返回新对象"。
*   `into_output(self)` 拿走了旧的 `Pin<Input>`，它就不再存在了。
*   返回的 `Pin<Output>` 是一个全新的类型。
*   这保证了旧状态对象不会继续被使用；状态是否可逆取决于 HAL 是否提供反向转换 API。

---

## 4. 对 STM32 项目的建议

在 `stm32_rust` (0 依赖) 项目中，我们稍后会实现 GPIO 模块。
我们将利用 Typestate 模式：
1.  定义 `mode::Input`, `mode::Output` 等空结构体。
2.  `GpioPin` 结构体携带 Mode 泛型。
3.  通过 `into_push_pull_output()` 等方法进行状态转换。

这将是 Day 2 的核心内容。

*注：本笔记解释了 Rust HAL 库如何通过类型系统防止硬件误操作。*
