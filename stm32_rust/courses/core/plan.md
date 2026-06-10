# Rust `core` 库 4 周学习计划

本计划基于以下已审校资料整理而成：

- `/home/zdz/mnt/Documents/Try/Rust/learning/base/inside-rust-std-library/notes/core-library-learning-roadmap.md`

目标不是“按文件顺序读完 `core` 源码”，而是围绕 `no_std`、STM32、MMIO、内部可变性、原子操作这些真实问题，建立一套可执行、可验证、可回扣到工程代码的学习路径。

当前项目的明确主线是：

1. 通过 Renode 完成仿真验证。
2. 在 `src/` 中逐步实现自己的 HAL 库。
3. 在 `examples/` 中编写完整业务示例，而不只是零散语法片段。
4. 通过 HAL 设计、寄存器封装、业务示例这三条线，反向学习 Rust `core` 模块。

## 学习目标

1. 建立 `core` / `alloc` / `std` 的边界认知。
2. 理解裸内存、指针、内部可变性、原子操作在 Rust 中的真实语义。
3. 搞清 Rust 常见语法糖背后的 Trait 协议。
4. 能把 `core` 源码里的设计回扣到 STM32 裸机代码，而不是停留在 API 使用层面。
5. 能把 `src/` 中的 HAL 抽象、`examples/` 中的业务流程和 Renode 下的验证结果串成完整闭环。

## 当前项目现状

根据当前代码库，计划执行时要以真实现状为准：

1. `run_renode.sh` 已存在，说明项目已经把 Renode 作为验证入口之一。
2. `src/` 下已有这些模块：
   - `gpio.rs`
   - `rcc.rs`
   - `time.rs`
   - `volatile.rs`
3. 当前 `gpio.rs`、`rcc.rs`、`time.rs` 仍然主要是占位实现。
4. `volatile.rs` 已经包含 `UnsafeCell + read_volatile/write_volatile` 这条核心 MMIO 封装线索。
5. `examples/` 当前只有 `day01_blinky.rs`，而且仍依赖外部 PAC / runtime 风格，不符合“项目内自研 HAL + 业务示例”的最终目标。

这意味着后续学习不能只停在读 `core` 源码，还必须同步推动两件事：

1. 把 `src/` 里的底层模块做成真正可复用的 HAL。
2. 把 `examples/` 变成基于本项目 HAL 的完整业务示例集合。

## 总体执行原则

后续每一天都要同时回答 3 个问题：

1. 今天读的 `core` 内容，解决了 HAL 里的什么设计问题？
2. 今天在 `src/` 里实现的抽象，准备被哪个 `examples/` 业务场景消费？
3. 今天写的 `examples/`，是否可以在 Renode 下被验证？

只学 `core` 不落代码，不合格。
只写寄存器代码不抽象成 HAL，不合格。
只写 HAL 不写业务示例，不合格。
写了示例但不能被 Renode 验证，也不合格。

## 执行规则

1. 每天学习 60 到 90 分钟。
2. 每次固定做三件事：
   - 先读公开 API 和文档注释。
   - 再看 1 到 2 个关键实现。
   - 最后写一个最小实验验证结论。
3. 不按文件顺序通读，不追求一次读懂全部宏和内部细节。
4. 所有结论尽量落到代码实验，不接受“看起来像这样”的印象式理解。

## 学习准备

### Day 0: 搭建环境与实验目录

* **目标**：把后续阅读与实验环境准备好。
* **内容**：
  * 安装源码：`rustup component add rust-src`
  * 安装工具：`cargo install cargo-expand cargo-asm`
  * 准备一个 `core_playground/` 目录
  * 建立 2 个最小练习 crate：
    * 一个普通 `std` crate，用来验证 Trait、Iterator、Option、Result
    * 一个 `no_std` crate，用来验证 `ptr`、`cell`、`atomic`、`arch`
* **产出**：
  * 可重复运行的实验目录
  * 一份固定笔记模板

---

## 第 1 周：内存、指针、类型系统地基

**目标**：先建立 `core` 最重要的底层语义认知。优先解决“内存怎么表示、指针怎么访问、类型系统怎么约束”这三个问题。

| 天数  | 阅读重点         | 必须搞清的问题                                                                 | 最小实验                                                  |
| :---  | :---             | :---                                                                           | :---                                                      |
| Day 1 | `core` 全局边界  | `core` / `alloc` / `std` 分别负责什么                                          | 列出常用 API 属于哪个 crate                               |
| Day 2 | `src/marker.rs`  | `Send` / `Sync` / `Unpin`、`Sized`、`PhantomData` 的区别                       | 写 3 个带 `PhantomData` / 裸指针的小类型，观察 trait 推导 |
| Day 3 | `src/mem/mod.rs` | `size_of`、`align_of`、`MaybeUninit`、`transmute` 的边界                       | 写一个数组分步初始化例子                                  |
| Day 4 | `src/ptr/mod.rs` | `read_volatile`、`write_volatile`、`addr_of!`、`read_unaligned` 各解决什么问题 | 手写最小 `VolatileCell<T>`                                |
| Day 5 | 周总结           | 能否解释未初始化、未对齐、volatile、别名规则的区别                             | 写一页“内存模型小抄”                                      |

### 第 1 周细化要求

**Day 1：`core` 的边界**

* 确认 `core` 不依赖堆，也不依赖 OS。
* 确认 `Vec<T>`、`String` 属于 `alloc`，不是 `core`。
* 确认 `for`、`?`、运算符重载等高级语法，最终都会落到 `core` 里的 Trait 协议。

**Day 2：`marker.rs`**

* 不要把 `Send` / `Sync` / `Unpin`、`Sized`、`PhantomData` 混成一类。
* 重点理解：
  * 什么是 Auto Traits
  * 什么是编译器特殊 Trait
  * `PhantomData` 为什么能影响生命周期、drop check、variance

**Day 3：`mem/mod.rs`**

* 重点不是背 API，而是搞清：
  * 未初始化内存为什么不能直接读
  * `MaybeUninit<T>` 为什么是标准且推荐的表达方式
  * `transmute` 为什么对大小、布局、有效值集合敏感

**Day 4：`ptr/mod.rs`**

* 重点区分四件事：
  * 普通读写
  * volatile 读写
  * unaligned 读写
  * 避免创建非法引用
* 要明确：
  * `addr_of!` / `&raw` 只是避免先构造非法引用
  * 它们不能替代 `read_unaligned` / `write_unaligned`

**Day 5：周总结自测题**

1. 为什么 `core` 里有 `ptr`，但没有 `Vec<T>`？
2. `MaybeUninit<T>` 到底在表达什么？
3. `read_volatile` 和普通解引用读取的区别是什么？
4. `PhantomData` 不占内存，为什么还能影响类型系统？
5. 未对齐访问和 volatile 访问是同一类问题吗？

---

## 第 2 周：Trait 协议与语法糖

**目标**：把 Rust 常见“语法行为”还原成 Trait 协议，建立从语法到源码的映射能力。

| 天数 | 阅读重点 | 必须搞清的问题 | 最小实验 |
| :--- | :--- | :--- | :--- |
| Day 6 | `src/ops/mod.rs` 第一部分 | `Deref`、`DerefMut`、`Drop` 的职责边界 | 写一个最小智能指针包装类型 |
| Day 7 | `src/ops/mod.rs` 第二部分 | 运算符重载与 `Index` 的设计意图 | 给 `RegisterValue(u32)` 实现位运算 trait |
| Day 8 | `src/iter/mod.rs` | 迭代器为什么能零成本 | 写一个环形缓冲区迭代器 |
| Day 9 | `src/option.rs` 与 `src/result.rs` | `Option` / `Result` 组合风格与 `?` 的意义 | 把一段 if/else 错误处理改写成链式调用 |
| Day 10 | 周总结 | 语法糖如何落到 Trait | 写一页“语法糖到 Trait 对照表” |

### 第 2 周细化要求

**Day 6：`Deref` / `Drop`**

* 不要把一元 `*` 简化理解成“调用 `.deref()`”。
* 要明确：
  * `Deref` 主要参与方法解析与自动借用/自动解引用的人体工学
  * 一元 `*` 还包含语言内建的引用/指针解引用规则
* `Drop` 重点看确定性析构，不要只看语法。

**Day 7：运算符协议**

* 理解 `Add`、`Sub`、`Index`、`IndexMut` 只是协议，不是编译器魔法特判。
* 思考：什么时候实现运算符能提升表达力，什么时候会误导调用者。

**Day 8：迭代器协议**

* 只抓住一件事：`Iterator` 的核心是 `next()`。
* 其它大多数行为都建立在默认方法与组合器之上。

**Day 9：错误与空值处理**

* 确认 `#[must_use]` 为什么重要。
* 理解 `map`、`and_then`、`ok_or` 的组合风格。
* 理解 `?` 为什么适合嵌入式错误传播，而不是只把它当成语法便利。

**Day 10：周总结自测题**

1. 哪些你平时当成“语法”的东西，本质上是 Trait 协议？
2. `Deref` 的适用边界是什么？
3. 为什么说迭代器常常是零成本抽象？
4. `?` 展开的核心思想是什么？

---

## 第 3 周：内部可变性、共享状态、原子操作

**目标**：搞清 Rust 如何在不依赖 GC 的前提下处理共享状态，并把这套机制映射到中断、主循环、裸机同步上。

| 天数 | 阅读重点 | 必须搞清的问题 | 最小实验 |
| :--- | :--- | :--- | :--- |
| Day 11 | `src/cell.rs` 第一部分 | `UnsafeCell` 与 `Cell<T>` 的关系 | 写一个 `Cell<u32>` 计数器 |
| Day 12 | `src/cell.rs` 第二部分 | `RefCell<T>` 为什么会 panic | 写一个触发双重可变借用的例子 |
| Day 13 | 嵌入式共享状态专题 | `Cell` 为什么不是同步原语 | 比较 `Cell`、`Mutex<Cell<_>>`、`AtomicU32` |
| Day 14 | `src/sync/atomic.rs` | `Ordering` 与 target 原子能力 | 写一个状态标志位例子 |
| Day 15 | 周总结 | 内部可变性与并发安全的边界 | 写一页“共享状态选型表” |

### 第 3 周细化要求

**Day 11：`UnsafeCell` / `Cell`**

* `UnsafeCell` 是打破 `&T` 不可变假设的基础类型。
* `Cell<T>` 适合轻量、可复制、小范围内部可变状态。

**Day 12：`RefCell<T>`**

* 重点理解“编译期借用检查”和“运行时借用检查”的边界。
* 不要把 `RefCell` 当成并发同步工具。

**Day 13：嵌入式共享状态**

* 必须明确：
  * `Cell` 不是 `Sync`
  * `Cell` 不是中断共享变量的通用方案
  * 中断与主循环共享通常还要依赖临界区、`Mutex`、关闭中断或原子类型

**Day 14：原子操作**

* 重点不是死记 `Ordering` 名称，而是理解：
  * 可见性
  * 重排序约束
  * target 是否支持对应宽度的原子操作

**Day 15：周总结自测题**

1. `UnsafeCell` 为什么是内部可变性的根？
2. `Cell<T>` 和 `RefCell<T>` 分别解决什么问题？
3. 为什么 `Cell` 不能直接替代同步原语？
4. `AtomicU32` 和临界区各适合什么场景？
5. 什么叫“原子可用性取决于 target/CPU 能力”？

---

## 第 4 周：编译器桥梁与回扣工程

**目标**：只读最必要的 compiler bridge 内容，然后把前 3 周的理解回扣到你自己的 STM32 工程代码。

| 天数 | 阅读重点 | 必须搞清的问题 | 最小实验 |
| :--- | :--- | :--- | :--- |
| Day 16 | `src/intrinsics/mod.rs` | intrinsic 和稳定 API 的边界 | 从一个稳定 API 反查对应 intrinsic 类别 |
| Day 17 | `src/arch/` | `asm!` 与架构专属指令的边界 | 写一个最小 `wfi` 或 barrier 示例 |
| Day 18 | 回看 `stm32_rust` 工程 | 哪些地方真正依赖 `ptr` / `cell` / `atomic` | 列一份工程改进清单 |
| Day 19 | 反向阅读法 | 从常用 API 追到 `core` 源码 | 选 5 个常用 API 回查实现 |
| Day 20 | 最终总结 | 能否形成自己的 `core` 地图 | 输出一份总总结文档 |

### 第 4 周细化要求

**Day 16：`intrinsics`**

* 只看函数签名和不稳定标记。
* 目标不是学会直接调用，而是知道稳定 API 底层是如何和编译器对接的。

**Day 17：`arch`**

* 只看和当前 STM32 场景强相关的内容。
* 不要被 x86、riscv、平台分支带偏。

**Day 18：回看工程**

回看这些真实问题：

1. 哪些寄存器访问应该使用 `read_volatile` / `write_volatile`？
2. 哪些封装需要 `UnsafeCell`，哪些根本不需要？
3. 哪些地方真的需要 `repr(C)`，哪些只是习惯性加上？
4. 哪些共享状态应改用临界区或原子类型？

**Day 19：反向阅读入口建议**

建议从这 5 个 API 反查：

1. `Option::map`
2. `Result::and_then`
3. `Cell::set`
4. `ptr::read_volatile`
5. `Iterator::map`

**Day 20：最终输出要求**

最终总结至少回答这 6 个问题：

1. 为什么 `UnsafeCell` 是内部可变性的根？
2. `Cell` 和 `AtomicU32` 的本质区别是什么？
3. 为什么不能把 `Deref` 等同于一元 `*`？
4. 为什么 `addr_of!` 不能替代 `read_unaligned`？
5. 为什么 `Option<&T>` 能和裸指针一样大？
6. 为什么 `core` 能支撑大量高级语法，却自己不依赖堆和 OS？

---

## 每日固定输出模板

```md
## 今天读了什么
- `core::ptr`
- `read_volatile`
- `addr_of!`

## 我确认的事实
- `addr_of!` 只是避免先创建引用
- 未对齐访问仍要看 `read_unaligned`

## 我还不确定的点
- `&raw` 和 `addr_of!` 在不同语境下的细粒度区别

## 我做的实验
- 写了一个最小 `VolatileCell<u32>`
```

## 优先级建议

如果当前目标是服务 `stm32_rust` 这个裸机工程，不建议平均分配时间。推荐优先级如下：

1. `src/ptr/mod.rs`
2. `src/cell.rs`
3. `src/mem/mod.rs`
4. `src/sync/atomic.rs`
5. `src/marker.rs`
6. `src/ops/mod.rs`
7. `src/option.rs` / `src/result.rs`
8. `src/iter/mod.rs`
9. `src/intrinsics/mod.rs`
10. `src/arch/`

这条顺序最贴近当前 STM32、MMIO、裸机启动、共享状态和 `no_std` 的真实问题。

## HAL / 示例 / Renode 三线映射

为了避免“学的是 `core`，做的是另一套东西”，后续开发要按下面三线并行推进：

| 阶段 | `core` 学习重点 | `src/` HAL 目标 | `examples/` 业务目标 | Renode 验证目标 |
| :--- | :--- | :--- | :--- | :--- |
| Phase 1 | `ptr` / `mem` / `cell` | 完成 `volatile`、`rcc`、`gpio` 最小可用封装 | `blinky`、GPIO 输出控制 | 能稳定点灯或翻转状态 |
| Phase 2 | `ops` / `option` / `result` | 完成 GPIO 配置 API、错误类型、基础时间抽象 | 按键输入、LED 模式切换 | 能观察到输入输出逻辑正确 |
| Phase 3 | `atomic` / `cell` / `marker` | 完成共享状态、延时、轮询或中断边界抽象 | 简单任务状态机、软件定时任务 | 能验证状态切换与时序 |
| Phase 4 | `intrinsics` / `arch` | 完成启动链路细化、屏障、必要内联汇编封装 | 综合示例：时钟 + GPIO + 定时 + 状态控制 | 能形成完整演示流程 |

## `src/` 的实现要求

后续在 `src/` 中实现 HAL 时，必须满足下面要求：

1. 不要把寄存器操作散落在 `main.rs` 或 `examples/` 中。
2. 寄存器访问应尽量收敛到 HAL 模块内部。
3. `volatile`、寄存器块、位操作、共享状态边界，要通过 `core` 中的抽象解释清楚。
4. 每新增一个 HAL 模块，都要能回答：
   - 它依赖了 `core` 的哪些模块？
   - 它隐藏了哪些 `unsafe`？
   - 它准备被哪个业务示例使用？

## `examples/` 的实现要求

后续在 `examples/` 中，不再只写“概念演示”，而要写完整业务路径示例。

最低要求：

1. 每个示例都必须明确依赖本项目 `src/` 中的 HAL。
2. 每个示例都要有业务目标，而不是只有单个寄存器操作。
3. 每个示例都要说明它在验证哪个 `core` 概念。
4. 每个示例都应尽量能被 Renode 运行验证。

建议的业务示例方向：

1. `day01_blinky`：最小点灯，验证 `volatile`、寄存器映射、RCC + GPIO 链路
2. `day02_button_led`：输入驱动输出，验证 GPIO 配置状态与轮询逻辑
3. `day03_led_pattern`：多状态切换，验证 `Option` / `Result` / 状态抽象
4. `day04_soft_timer`：软件延时与任务切换，验证 `time`、循环、共享状态

## Renode 验证要求

既然当前项目已经使用 Renode，就把它视为学习闭环的一部分，而不是附属脚本。

后续每一阶段至少要回答：

1. 当前示例是否能被 Renode 启动？
2. Renode 下能观察到什么可验证现象？
3. 如果验证失败，是 HAL 问题、示例业务问题，还是对 `core` 语义理解错误？

因此，后续新增示例时，默认要补全：

1. 示例入口
2. 依赖的 HAL 模块
3. 预期的仿真现象
4. 最小验证步骤

## 完成标准

学完不等于“看过源码”，而是至少达到下面 4 条：

1. 能解释 `core` / `alloc` / `std` 的边界。
2. 能说明 `volatile`、unaligned、内部可变性、原子操作分别解决什么问题。
3. 能把 `Deref`、`Iterator`、`Option`、`Result` 这些抽象映射回 Trait 协议。
4. 能回过头改进 `stm32_rust` 里的寄存器访问和共享状态封装。
5. 能形成一条“HAL 实现 -> 示例业务 -> Renode 验证 -> 回扣 `core` 语义”的闭环。

---

## 自我审查

列出计划之后，不能只看“有没有学完”，还要检查自己有没有学偏。下面这部分是执行本计划时必须做的自我审查。

### 每天学习后的自查

1. 今天的结论，是否来自源码、文档注释或实验，而不是凭印象复述？
2. 我今天区分清楚“语言规则”和“库层抽象”了吗？
3. 我有没有把两个不同问题混成一个问题？
4. 我写的实验，是否真的验证了结论，而不是只证明“代码能编译”？
5. 我能不能把今天的知识点回扣到 STM32 / `no_std` 工程里的一个真实场景？

### 每周结束后的自查

1. 这一周我掌握的是“API 名字”，还是“语义边界”？
2. 我是否能说清“这个 API 解决什么问题”，同时也说清“它不解决什么问题”？
3. 我是否把 `core` 里的概念错误迁移成了嵌入式工程实践？
4. 本周是否至少留下了 1 个可复现的小实验和 1 页自己的总结？
5. 如果把本周内容讲给别人听，我是否会说出绝对化、过度简化、版本不准确的话？

### 核心误区审查清单

执行过程中，反复检查自己有没有掉进下面这些坑：

1. 有没有把 `Send` / `Sync` / `Unpin`、`Sized`、`PhantomData` 混成同一种东西？
2. 有没有把 `MaybeUninit<T>` 理解成“Rust 版 `undefined`”，却忽略了它是受类型系统约束的显式抽象？
3. 有没有把 `addr_of!` / `&raw` 当成“万能安全指针工具”，却忘了 unaligned 访问还要看 `read_unaligned` / `write_unaligned`？
4. 有没有把 volatile、unaligned、aliasing、interior mutability 混成一个统称？
5. 有没有把 `Deref` 讲成“一元 `*` 的底层实现”？
6. 有没有把 `Cell` 当成中断共享状态的默认答案？
7. 有没有把 `RefCell` 当成同步原语，而不是运行时借用检查工具？
8. 有没有把原子操作讲成“任何 target 都天然可用”？
9. 有没有把 `repr(C)` 讲成“只要碰寄存器就必须加”？
10. 有没有因为源码里出现某个内部接口，就误以为业务代码应该直接调用它？

### 实验质量审查

每做完一个实验，至少问自己下面 5 个问题：

1. 这个实验验证的是语义，还是只验证了语法？
2. 如果把优化等级改成 `release`，结论还成立吗？
3. 如果把场景换成 `no_std`，这个结论还成立吗？
4. 如果把场景换成中断共享、MMIO、未对齐访问，实验是否仍覆盖关键边界？
5. 我有没有故意构造一个反例，证明自己不是只看到了“正常路径”？

### 面向 `stm32_rust` 工程的专项审查

学完每个阶段后，都要回看 `stm32_rust` 工程，检查这些问题：

1. 当前寄存器访问是否真的使用了正确的 `volatile` 语义？
2. 当前封装是否错误地创建了本不该创建的引用？
3. 当前某些 `unsafe` 是否其实只是因为没有把 `core` 的抽象学透？
4. 当前共享状态是否错误地依赖 `Cell` 或 `static mut`？
5. 当前是否有地方机械地加了 `repr(C)`，但其实并没有固定布局需求？

### 收尾审查

整份计划执行完后，最后做一次总自查：

1. 我现在能否用自己的话解释 `core` / `alloc` / `std` 的真实边界？
2. 我能否清楚区分这四类问题：
   - 内部可变性
   - volatile 访问
   - 未对齐访问
   - 并发同步
3. 我能否指出 `stm32_rust` 工程里 3 处可以被这轮学习直接改进的地方？
4. 我是否还在使用“看起来差不多”“一般来说就是这样”这类不可靠表述？
5. 如果现在重新读 `core` 源码，我是否已经有一张明确的问题地图，而不是再次线性乱读？

### 自我审查结论模板

```md
## 本轮自我审查结论

### 我确认已经掌握的点
- 
- 

### 我仍然模糊的点
- 
- 

### 我发现自己最容易混淆的概念
- 
- 

### 我准备怎么补救
- 重新读哪个模块：
- 重做哪个实验：
- 回看工程里的哪个场景：
```
