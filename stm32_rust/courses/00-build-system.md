# Day 0 - 构建系统与工具链深度解析

本节课旨在通过对比 **Zig** 和 **Go** 的构建机制，深入理解 Rust 嵌入式项目的核心配置文件。
我们将探讨这些配置如何一步步塑造最终生成的二进制文件（`.elf`），以及当我们移除所有第三方依赖后，还需要手动处理哪些底层逻辑。

---

## 1. 核心配置文件概览

在 Rust 嵌入式项目中，构建系统由以下三个核心文件驱动：

| 文件                     | 职责                                                       | Zig 对应物                                     | Go 对应物                 |
| :---                     | :---                                                       | :---                                           | :---                      |
| **`Cargo.toml`**         | 声明依赖、特性门控 (Features)、优化配置 (Profiles)         | `build.zig` (部分配置)、`build.zig.zon` (依赖) | `go.mod`                  |
| **`.cargo/config.toml`** | 配置工具链路由：目标架构 (Target)、链接器、运行器 (Runner) | `build.zig` 中的 `target` 设置                 | 环境变量 `GOOS`, `GOARCH` |
| **`build.rs`**           | 编译前脚本：处理链接脚本、生成代码、设置环境变量           | `build.zig` (构建逻辑本身)                     | `//go:generate` 脚本      |

---

## 2. `Cargo.toml`：依赖声明与二进制裁剪

### 2.1 关键配置项
```toml
[package]
name = "stm32_rust"
edition = "2021"

[dependencies]
# 零依赖模式：这里留空，不引入 cortex-m-rt, stm32f1 等
# cortex-m = ... 
# panic-halt = ...

[profile.release]
opt-level = "z"      # 优先优化体积
lto = true           # 启用链接时优化 (LTO)
codegen-units = 1    # 单代码生成单元，最大化 LTO 效果
panic = "abort"      # Panic 时直接 abort，节省 unwind 代码体积
```

### 2.2 对二进制的影响
1.  **Profile 优化**：
    *   `opt-level = "z"` 告知编译器使用 `-Oz` 标志，显著减小 `.text` 段体积。
    *   `panic = "abort"` 避免引入 unwind 相关代码，是裸机嵌入式中最常见的选择，可减小体积并简化运行时要求。

### 2.3 对比 Zig / Go
*   **Zig**: 优化级别在 `build.zig` 中通过 `exe.setOptimizeMode(.ReleaseSmall)` 设置。Zig 没有 Rust 式 stack unwinding；panic 通常走 trap/abort 或用户自定义 panic handler。
*   **Go**: TinyGo 默认带有 Panic 打印机制（即使不处理也会打印 panic 信息），很难完全剥离，除非使用极其底层的 `//go:export` 绕过 runtime。

---

## 3. `.cargo/config.toml`：工具链路由

### 3.1 关键配置项
```toml
[build]
# 目标架构：ARM Cortex-M3, 无 OS (none), 无硬件浮点 (eabi)
target = "thumbv7m-none-eabi"

[target.thumbv7m-none-eabi]
# 运行器：`cargo run` 时自动执行烧录和启动
runner = "probe-rs run --chip STM32F103C8Tx"
# 链接器：使用 LLVM lld，比系统 ld 对嵌入式支持更好
linker = "rust-lld"
```

### 3.2 对二进制的影响
1.  **指令集架构**：决定了生成 Thumb-2 指令集。
2.  **目标平台选择**：告诉 `rustc` 生成 Cortex-M3 Thumb 指令、使用裸机 ABI。是否链接 `std` 由源码中的 `#![no_std]` 决定；在该 target 下通常只能使用 `core`/`alloc` 这类不依赖 OS 的库。

### 3.3 对比 Zig / Go
*   **Zig**: 在 `build.zig` 中配置目标：
    ```zig
    exe.setTarget(.{ .cpu_arch = .thumb, .os_tag = .freestanding, .abi = .eabi });
    ```
    Zig 的编译器内置了各种 target 的定义，不需要像 Rust 那样通过 rustup 安装 target JSON 描述文件。
*   **Go**: 通过环境变量 `GOARCH=arm GOARM=7` 设置。

---

## 4. `build.rs` 与 `memory.x`：内存布局注入

### 4.1 关键流程
嵌入式开发必须定义内存布局。在零依赖模式下，这完全由你控制。

**`build.rs` 脚本**：
```rust
fn main() {
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    
    // 将 memory.x 复制到构建输出目录
    std::fs::copy("memory.x", out.join("memory.x")).unwrap();

    // 告诉链接器使用它
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rustc-link-arg=-Tmemory.x");
}
```

**`memory.x` 内容** (零依赖版)：
```ld
MEMORY
{
  FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 64K
  RAM (rwx)  : ORIGIN = 0x20000000, LENGTH = 20K
}
_estack = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
  .vector_table : {
    LONG(_estack);            /* 栈顶 */
    KEEP(*(.vector_table.reset_vector)); /* 复位向量 */
  } > FLASH
  /* ... .text, .data, .bss ... */
}
```

### 4.2 对二进制的影响
*   **向量表位置**：链接脚本决定向量表必须放在 Flash 起始地址 `0x08000000`。
*   **段初始化**：脚本定义了 `_sdata`, `_edata`, `_sidata` 等符号，供我们在 `main.rs` 中手动编写循环来清零 BSS 和拷贝 Data。

### 4.3 对比 Zig / Go
*   **Zig**: Zig 不需要 `build.rs` 这种中间脚本来传递链接脚本路径。直接在 `build.zig` 调用 `exe.setLinkerScript(b.path("memory.x"));` 即可。Zig 的构建系统是图结构，处理依赖和链接参数比 Rust 的 "print 字符串" 模式更直观。
*   **Go**: TinyGo 使用 JSON target 文件定义链接脚本路径，用户通常不直接干预。

---

## 5. 启动代码：零依赖下的核心差异 (Startup Code)

当我们移除了 `cortex-m-rt`，进入**零依赖 (Zero-Dependency)** 模式时，**Rust** 和 **Zig (freestanding)** 的表现惊人地一致，都需要手动处理启动流程。

| 任务                     | Rust (0-dep)                                    | Zig (freestanding)                                                         | Go (TinyGo)      |
| :---                     | :---                                            | :---                                                                       | :---             |
| **向量表**               | `#[link_section]` 手动放置数组                  | `comptime` 数组 + `export` 放置                                            | 自动内置，不可见 |
| **栈初始化**             | 链接脚本设置 `_estack`，CPU 自动加载            | 链接脚本设置，CPU 自动加载                                                 | Runtime 隐藏处理 |
| **BSS 清零 / Data 拷贝** | **必须手写** (`ptr::write_bytes` / `ptr::copy`) | **必须手写** (或使用 `std.start` 的部分逻辑，但在 freestanding 下通常手写) | Runtime 自动处理 |
| **Panic 处理**           | `#[panic_handler]` 必须实现                     | `std.debug.panic` (可自定义)                                               | Runtime 内置     |

**重要澄清**：
*   在 OS 环境下，Zig 的 `std.start` 确实会帮你处理 Args 解析和 Panic 捕获。
*   但在 **`freestanding` (裸机)** 环境下，Zig `std` 中依赖 OS 的部分不可用，且**不会**自动生成完整 MCU 启动代码。平台无关的工具模块仍可使用；向量表、BSS/Data 初始化通常仍需要项目自己处理。
*   这与 Go (TinyGo) 形成鲜明对比，后者面向裸机时通常仍带有自己的 runtime 支持；与手写 Rust/Zig 启动代码相比，底层初始化细节较多被 runtime 隐藏。

---

## 6. 总结：从配置到二进制的完整链路

1.  **定义布局**：`memory.x` 定义 Flash/RAM，`build.rs` 将其传给链接器。
2.  **手写启动**：`src/main.rs` 实现 `ResetHandler`，处理向量表和内存初始化。
3.  **编译代码**：`rustc` (通过 `target` 设置) 编译为 `thumbv7m` 架构。
4.  **链接**：`lld` 将代码段放入 Flash，数据段放入 RAM。
5.  **烧录**：`probe-rs` 将 `.elf` 写入芯片。

这个链路展示了在没有第三方库“魔法”的帮助下，Rust 是如何通过配置和少量底层代码控制硬件的。

---

*下一步：Day 1 将基于此环境，进行第一次寄存器操作验证。*
