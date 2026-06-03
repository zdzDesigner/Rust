# 06 - Zig Packed Struct 的跨越 (Spanning) 机制

在之前的讨论中，我们提到了 Zig 支持位域。通过观察 `chip.zig` 源码，我们发现了一个更强大的特性：**跨边界自动连续（Spanning）**。

---

## 1. Zig 的“流式”位打包

当你定义一个 `packed struct(u32)` 时，Zig 允许字段**跨越** 32 位边界。它不会在填满一个 `u32` 后强制填充 Padding 或断开，而是像水流一样自然流入下一个 `u32` 的高位/低位。

### 示例代码
```zig
// 定义一个跨越边界的结构体
const SpanningRegs = packed struct(u32) {
    field_a: u20, // 占用 Word 0 的 bit 0-19
    
    // field_b 将跨越 Word 0 和 Word 1
    // 占用 Word 0 的 bit 20-31 (12 bits)
    // 占用 Word 1 的 bit 0-7 (8 bits)
    field_b: u20, 
    
    field_c: u4,  // 继续占用 Word 1 的 bit 8-11
};
```
**底层行为**：
Zig 编译器会自动计算位偏移。`field_b` 的值会被拆分存储在两个连续的 `u32` 中。访问 `field_b` 时，编译器生成的代码会自动执行位移和掩码操作来重组这个值。

---

## 2. 与 Rust 的彻底对比

这个特性是 Rust **完全无法企及**的。

| 特性 | Zig (`packed struct`) | Rust |
|:---|:---|:---|
| **子字节类型** | 支持 (`u4`, `u20` 等) | **不支持** (最小 `u8`) |
| **跨边界存储** | **支持**。字段可分裂存储 | **不支持**。字段必须在字节边界 |
| **自动重组** | 编译器自动生成读写代码 | 开发者必须手动编写位移逻辑 |
| **内存布局** | 极致紧凑，位级连续 | 字节对齐，存在 Padding |

### Rust 的痛苦现实
在 Rust 中，你甚至无法定义 `u20`。
如果你想模拟上面的结构，你必须手动定义：
```rust
// 手动拆分
struct SpanningRegs {
    word0: u32,
    word1: u32,
}

impl SpanningRegs {
    fn get_field_b(&self) -> u32 {
        // 手动从两个 u32 中拼凑出 20 位数据
        // 极其容易出错，且性能较差
        let low = (self.word0 >> 20) & 0xFFF;
        let high = self.word1 & 0xFF;
        (high << 12) | low
    }
}
```

---

## 3. 嵌入式意义

在 STM32 等芯片中，某些复杂的控制寄存器（如 Flash 控制寄存器、DMA 配置）往往包含大量非标准位宽的字段。
*   **Zig**：可以直接按照参考手册的位定义顺序，一个接一个地写下来，编译器保证物理内存布局一致。
*   **Rust**：只能依靠工具生成 (`svd2rust`)，或者开发者忍受繁琐的位移计算。

**结论**：Zig 的 `packed struct` 不仅仅是语法糖，它是对**硬件位级操作**的一等公民支持。这使得 Zig 在编写底层寄存器映射代码时，比 Rust 更直观、更接近硬件本质。

*注：本笔记补充了 Zig packed struct 跨越边界的特性，这是 Rust 所不具备的。*
