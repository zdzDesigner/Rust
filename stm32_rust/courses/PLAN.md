# STM32F103 Rust HAL 库开发与学习路线图

本项目旨在通过**从零编写 STM32F1 的 HAL 库**来系统学习 Rust 嵌入式开发。
我们将以 **Zig 项目经验**为参照，对比两者的内存管理、抽象机制与工具链。

## 📂 项目结构目标

```text
stm32_rust/
├── courses/             # <--- 教程与规划目录
│   └── PLAN.md          # 本计划文件
├── src/                 # HAL 库源码 (Library Crate)
│   ├── lib.rs           # 库入口
│   ├── rcc.rs           # 时钟树抽象
│   ├── gpio.rs          # GPIO 抽象 (重点：Typestate)
│   └── ...
├── examples/            # 验证示例 (Binary Crates)
│   ├── day01_blinky.rs
│   └── ...
├── Cargo.toml           # 构建配置
├── build.rs             # 构建脚本
└── memory.x             # 内存布局
```

## 📅 14 天学习与开发路线图

### Day 0: 构建系统与工具链深度解析 (Go/Zig 对照)
*   **目标**：清晰理解配置如何塑造二进制。
*   **内容**：
    *   **Cargo.toml**：依赖声明、特性门控 (`features`)、Profile (`release`/`dev`)。
    *   **.cargo/config.toml**：目标架构 (`thumbv7m-none-eabi`)、运行器 (`probe-rs`)。
    *   **build.rs**：链接脚本处理 (`memory.x`)、编译期代码生成。
*   **对照**：
    *   Rust `Cargo.toml` + `build.rs` vs Zig `build.zig` vs Go `go.mod` + `//go:generate`。

### Phase 1: 裸机与寄存器 (Day 1-4)

| 阶段 | 天数 | Rust 核心概念 (Concept) | HAL 库开发任务 (`src/`) | 验证示例 (`examples/`) | Zig 对照点 |
|:---|:---:|:---|:---|:---|:---|
| **Phase 1** | Day 1 | `no_std` 入口、`volatile` 指针操作 | 手写最小 PAC 寄存器定义、配置环境 | `day01_blinky.rs` (点灯) | `@ptrFromInt` vs `volatile` |
| | Day 2 | **类型状态机 (Typestate)**、`Into<T>` / `From<T>` | 实现 `gpio.rs`：引脚模式转换 | `day02_gpio_state.rs` (按键输入) | Zig 显式状态枚举 |
| | Day 3 | 时钟树 (RCC)、所有权传递 | 实现 `rcc.rs`：总线时钟管理 | `day03_rcc_led.rs` (先开时钟再点灯) | Zig 显式初始化结构体 |
| | Day 4 | 中断 (NVIC)、`#[interrupt]` 宏 | 封装中断向量、临界区 | `day04_exti_button.rs` (外部中断) | `comptime` 中断表 |

### Phase 2: 抽象与特性 (Day 5-7)

| 阶段 | 天数 | Rust 核心概念 (Concept) | HAL 库开发任务 (`src/`) | 验证示例 (`examples/`) | Zig 对照点 |
|:---|:---:|:---|:---|:---|:---|
| **Phase 2** | Day 5 | `Timer`、`Duration` 抽象 | 实现 `timer.rs`：延时与定时 | `day05_timer_blink.rs` (精准延时) | `std.time` |
| | Day 6 | `Serial`、`fmt::Write` 接口 | 实现 `serial.rs`：实现 `Write` trait | `day06_uart_hello.rs` (打印日志) | `std.io.Writer` |
| | Day 7 | **错误处理 `Result<T, E>`** | 定义 `Error` 枚举、类型别名 | `day07_error_handling.rs` | Error Union |

### Phase 3: 并发与零拷贝 (Day 8-10)

| 阶段 | 天数 | Rust 核心概念 (Concept) | HAL 库开发任务 (`src/`) | 验证示例 (`examples/`) | Zig 对照点 |
|:---|:---:|:---|:---|:---|:---|
| **Phase 3** | Day 8 | 替代 `static mut` 的安全方案、`Mutex` | 封装全局变量安全访问 | `day08_static_log.rs` (全局日志) | `std.Thread.Mutex` |
| | Day 9 | DMA、所有权转移 API | 实现 `dma.rs`：基于所有权的传输 | `day09_dma_serial.rs` (零拷贝发送) | `*volatile` 传递 |
| | Day 10 | `Send`/`Sync`、原子操作 | 实现 `atomic.rs`：无锁缓冲区 | `day10_ring_buffer.rs` | `std.atomic` |

### Phase 4: 现代异步架构 (Day 11-14)

| 阶段 | 天数 | Rust 核心概念 (Concept) | HAL 库开发任务 (`src/`) | 验证示例 (`examples/`) | Zig 对照点 |
|:---|:---:|:---|:---|:---|:---|
| **Phase 4** | Day 11 | `async`/`await` 基础、Future | 引入 `embassy` 或手写极简 Executor | `day11_async_blink.rs` | `async`/`await` |
| | Day 12 | 硬件异步 (I2C/SPI) | 实现 `i2c.rs` 异步驱动 | `day12_i2c_sensor.rs` (读取 MPU6050) | 异步 IO |
| | Day 13 | 性能分析、内联汇编 | 编写 Benchmark 工具 | `day13_perf_test.rs` | `std.bench` |
| | Day 14 | CI、文档测试、Crate 发布 | 完善 `README`、文档 | 最终综合演示 | `build.zig` vs `Cargo.toml` |

---

### 💡 学习建议

1.  **先跑通 Day 1**：确保 `cargo run --example day01_blinky` 能点亮板载 LED。
2.  **对照 Zig**：在写 Rust 的 `gpio.rs` 时，时刻回想你在 Zig 里是如何处理引脚状态的，体会两者在"类型安全"与"显式控制"上的差异。
3.  **不要跳过报错**：Rust 的编译器报错通常包含详细的修复建议（Help），这是学习语言规则最好的方式。
4.  **关注二进制体积**：经常检查 `target/thumbv7m-none-eabi/release/stm32_rust.elf` 的大小，理解 `lto` 和 `opt-level` 的效果。

---
*Created by ruster on 2024-05-29*
