# 02 - 寄存器位域 (Bitfields) 处理差异

在嵌入式开发中，寄存器往往不是按字节划分的，而是按 **位 (Bit)** 划分的（如一个 u32 寄存器的前 4 位是配置 A，接下来的 8 位是配置 B）。

（作者评价：这是 Zig 对 Rust 形成体验优势最显著的领域之一，主要体现在语法的直观性上。）

---

## 1. Zig 的原生位域支持 (Bit-fields)

Zig 提供了 **任意位宽整数** (`u1` 到 `u65535`) 和 **紧凑结构体 (`packed struct`)**，允许你直接以位为单位定义寄存器。

### Zig 代码示例
```zig
// 定义一个 32 位寄存器，精确映射每一位
const BTR = packed struct(u32) {
    ADDSET: u4,    // 占用 bit 0-3
    ADDHLD: u4,    // 占用 bit 4-7
    DATAST: u8,    // 占用 bit 8-15
    // ... 编译器自动处理位移和掩码
};

// 使用：直接赋值，编译器自动生成位移和掩码代码
var reg: BTR = undefined;
reg.ADDSET = 5; // 概念上等价于对底层 32-bit 存储做 mask/shift 更新
```
**优势**：语义直观，编译器处理底层细节，零心智负担。

---

## 2. Rust 的现状：原生缺失

**Rust 目前没有原生位域语法，也没有非 8 倍数的整数类型（最小是 `u8`）。**

这意味着在 Rust 中，你**无法**写出与 Zig 等价的结构体定义。

### Rust 中的替代方案

#### 方案 A：使用新类型包装 + 手动位移 (0 依赖推荐)
在你的 `stm32_rust` (0 依赖) 项目中，这是最标准的做法。
将整个寄存器视为一个 `u32`，通过方法暴露位段操作。

```rust
// 将整个寄存器包装为 u32
pub struct BtrReg(pub u32);

impl BtrReg {
    // Getter: 获取 ADDSET (0-3 位)
    pub fn addset(&self) -> u8 {
        (self.0 & 0xF) as u8
    }

    // Setter: 设置 ADDSET
    pub fn set_addset(&mut self, val: u8) {
        // 清除旧位，写入新位 (需确保 val 不超过 4 位)
        self.0 = (self.0 & !0xF) | ((val as u32) & 0xF);
    }
}
```
*   **缺点**：代码量大，容易写错位移逻辑（如把 `0xF` 写成 `0xFF`）。
*   **优点**：0 依赖、可控性强、生成结果直观。

#### 方案 B：使用第三方宏 (`modular-bitfield`)
社区标准做法是使用宏来生成上述代码。
```rust
use modular_bitfield::prelude::*;

#[bitfield]
pub struct BtrReg {
    pub addset: B4,   // 借用 crate 定义的 B4 类型
    pub addhld: B4,
    pub datast: B8,
    // ...
}
```
*   **缺点**：引入了外部依赖和宏展开黑盒。

---

## 3. 核心概念对比：位域 vs 紧凑布局

| 特性 | Zig | Rust | 说明 |
| :--- | :--- | :--- | :--- |
| **基础类型** | `u1` ~ `u65535` | `u8`, `u16`, `u32`... | Zig 可直接定义任意位宽整数，Rust 稳定版不能 |
| **位域声明** | `packed struct` + 任意位宽整数 | 无原生语法 | Rust 需要手写位运算或依赖宏 crate |
| **紧凑布局** | 支持按位压缩字段 | `#[repr(packed)]` 仅影响对齐与字段间 padding | Rust 的 `packed` 不是 bitfield |

### 关键误区
*   **Zig `packed struct`**：可以把字段按位紧凑排列，字段可占用非整数字节。
*   **Rust `#[repr(packed)]`**：会降低对齐要求并最小化字段间 padding，但**不会**把普通字段压成位域，也**不能**声明 `u4` 这样的字段类型。
*   如果需要精确的寄存器位段语义，Rust 通常要靠手写 mask/shift，或使用 `modular-bitfield` 这类 crate。

---

## 4. 为什么 Rust 的 PAC 代码那么长？

因为 Rust 语言本身没有原生 bitfield 语法，svd2rust 通常生成 field reader/writer API 来封装位操作并维持类型安全。这通常会生成较冗长的字段访问 API，而 Zig 只需要 `reg.ADDSET = 5;`。

*注：本笔记补充了关于位域处理的语言级差异，是编写底层寄存器映射的重要参考。*
