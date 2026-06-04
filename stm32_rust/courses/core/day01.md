# Day 1: 建立 `core` / `alloc` / `std` 的边界

## 学习目标

今天不进入复杂源码细节，只解决一个基础但决定后续阅读方向的问题：

1. `core` 到底提供什么，不提供什么。
2. `alloc` 在 `core` 之上补了什么能力。
3. `std` 又在前两者之上补了什么能力。
4. 为什么 STM32 裸机工程可以不用 `std`，但仍然能写出很多看起来“很高级”的 Rust 代码。

学完今天的内容后，你应该能避免一个最常见的误区：把“Rust 标准库”模糊地全算成 `std`。

---

## 先建立总图

可以先把 Rust 常用能力粗分成三层：

```text
Rust 语言能力
    |
    +-- core   : 无堆、无 OS，提供最基础的类型、trait、指针、内存、控制流抽象
    +-- alloc  : 在 core 之上增加堆分配能力
    +-- std    : 在 core + alloc 之上增加 OS 相关能力
```

这三层不是“谁更高级谁替代谁”的关系，而是逐层叠加：

1. `core` 是最小基础层。
2. `alloc` 依赖 `core`，但不要求操作系统。
3. `std` 依赖 `core` 和 `alloc`，并额外依赖操作系统环境。

对 STM32 裸机项目来说，最关键的结论是：

1. 你通常没有 `std`。
2. 你一定会依赖 `core`。
3. 你是否能用 `alloc`，取决于你是否实现或引入了 allocator。

---

## `core` 是什么

`core` 是 Rust 最基础的库层能力集合。它不依赖堆，也不依赖 OS，但足以支撑大量语言特性和基础抽象。

### `core` 里通常有什么

1. 基础 trait
   - `Copy`
   - `Clone`
   - `Sized`
   - `Send`
   - `Sync`
   - `Unpin`

2. 基础枚举与控制流抽象
   - `Option<T>`
   - `Result<T, E>`

3. 指针与内存原语
   - `core::ptr`
   - `core::mem`
   - `core::slice`

4. 内部可变性与低层同步基础
   - `core::cell`
   - `core::sync::atomic`

5. 语法糖背后的协议
   - `core::ops`
   - `core::iter`

### `core` 里通常没有什么

1. 堆分配容器
   - `Vec<T>`
   - `String`
   - `Box<T>`

2. OS 能力
   - 线程
   - 文件系统
   - 网络
   - 进程
   - 计时器的标准 OS 封装

所以，`core` 不是“阉割版 Rust”，而是“去掉堆和 OS 依赖后的 Rust 基础设施层”。

---

## `alloc` 是什么

`alloc` 建立在 `core` 之上，负责把“堆分配能力”接进来。

### `alloc` 常见内容

1. `Vec<T>`
2. `String`
3. `Box<T>`
4. `Rc<T>`
5. 各类依赖 heap 的集合和智能指针

### 关键点

1. `alloc` 不等于 `std`。
2. `alloc` 不要求 OS，但要求有可用的 allocator。
3. 在某些嵌入式项目里，可以是 `no_std + alloc`，而不是只有 `no_std` 或只有 `std` 两种世界。

这点很重要，因为很多初学者会误以为：

- 不能用 `std` = 不能用堆

这个结论并不总是成立。真正准确的说法是：

- 不能用 `std` 不代表不能用堆
- 能不能用堆，要看 allocator 是否存在

---

## `std` 是什么

`std` 建立在 `core + alloc` 之上，再加上操作系统相关封装。

### `std` 常见能力

1. 线程
   - `std::thread`
2. 文件系统
   - `std::fs`
3. 网络
   - `std::net`
4. 进程与环境
   - `std::process`
   - `std::env`
5. 时间与 IO 等标准运行时接口

所以你平时在桌面环境里写 Rust，觉得“什么都有”，主要是因为 `std` 帮你把：

1. `core` 的基础能力
2. `alloc` 的堆能力
3. OS 的系统接口

一起打包暴露出来了。

---

## 为什么 `no_std` 里仍然能写很多高级 Rust

这是今天最重要的认知点之一。

很多人第一次接触裸机 Rust，会误以为不用 `std` 就只能写很原始的代码。但这不对。

即使在 `no_std` 下，你通常仍然可以使用：

1. `Option<T>`
2. `Result<T, E>`
3. `Cell<T>`
4. `MaybeUninit<T>`
5. `Iterator` 相关协议
6. 运算符 trait
7. `read_volatile` / `write_volatile`

原因很简单：这些能力属于 `core`，不依赖 OS。

所以真正的边界不是：

- “高级 Rust” vs “低级 Rust”

而是：

- 是否依赖堆
- 是否依赖 OS

这也是为什么后面学习 `core` 源码，对 STM32 / `no_std` 项目尤其重要。

---

## 为什么很多语法糖最终会落到 `core`

今天先建立总印象，不深挖 lowering 细节。

### 例子 1：`for`

`for` 循环背后依赖的是迭代器协议，也就是 `IntoIterator` / `Iterator`。

### 例子 2：运算符

`+`、`-`、`[]` 这类运算符行为，背后依赖的是 `core::ops` 里的 trait。

### 例子 3：错误传播

`Option` / `Result` 的组合风格以及 `?` 这类语法行为，底层也依赖 `core` 的抽象能力。

今天不要求你把每个语法糖的编译器展开路径完整背下来，但你必须先建立这个方向感：

1. Rust 很多“像语言内建魔法”的东西
2. 实际上都和 `core` 里的 trait、枚举、基础抽象密切相关

---

## 和 `stm32_rust` 工程的直接关系

把今天的知识直接回扣到当前工程，可以得到 4 个非常实际的结论：

1. 你的裸机工程可以不用 `std`，但仍然能用 `Option`、`Result`、`Cell`。
2. 寄存器访问相关 API 会出现在 `core::ptr`，因为它们不需要 OS。
3. 后面真正值得优先读的，是 `core::ptr`、`core::mem`、`core::cell`、`core::sync::atomic`。
4. `std::fs`、`std::thread`、`std::net` 这类模块，对当前 STM32 学习主线帮助很小，不该优先投入精力。

---

## 今天的学习步骤

按下面顺序执行，不要跳。

### 第 1 步：读计划与路线图

阅读：

1. `/home/zdz/Documents/Try/Rust/learning/stm32_rust/courses/core/plan.md`
2. `/home/zdz/mnt/Documents/Try/Rust/learning/base/inside-rust-std-library/notes/core-library-learning-roadmap.md`

重点只看这些点：

1. `core` / `alloc` / `std` 的边界
2. 第 1 周 Day 1 的要求
3. 自我审查里的“每天学习后的自查”

### 第 2 步：列一张 API 归属表

至少列出 15 个你熟悉的 API，给它们分类到 `core` / `alloc` / `std`。

建议至少覆盖这些：

1. `Option`
2. `Result`
3. `Iterator`
4. `Copy`
5. `Clone`
6. `Deref`
7. `size_of`
8. `read_volatile`
9. `UnsafeCell`
10. `Vec`
11. `String`
12. `Box`
13. `Mutex`
14. `thread::spawn`
15. `fs::read_to_string`

### 第 3 步：写一个最小边界实验

目标不是做大工程，而是验证：哪些能力属于哪一层。

建议分两个最小文件：

1. 一个普通 `std` 小例子
2. 一个最小 `no_std` 小例子

在实验里分别尝试：

1. `core::mem::size_of`
2. `core::cell::Cell`
3. `core::ptr::read_volatile`
4. `alloc::vec::Vec`
5. `std::thread::spawn`

你要观察的重点不是“都跑通”，而是：

1. 哪些 API 天然不依赖 OS
2. 哪些 API 一出现就说明你已经越过了 `core` 边界

### 第 4 步：回扣当前工程

回答这 4 个问题：

1. 为什么 `stm32_rust` 可以不用 `std` 但仍然能使用 `Option`？
2. 为什么 `read_volatile` 在 `core` 里，而不是 `std` 里？
3. 如果以后你要实现寄存器封装，应该优先读哪些 `core` 模块？
4. `Vec<T>` 和 `String` 为什么不适合默认归到当前裸机工程主线里？

---

## 今日最小实验建议

### 实验 1：API 分层识别

写一张表：

```md
| API | 所属层 | 为什么 |
| :--- | :--- | :--- |
| `Option<T>` | `core` | 不依赖堆和 OS |
| `Vec<T>` | `alloc` | 依赖堆分配 |
| `std::thread::spawn` | `std` | 依赖线程和 OS 运行时 |
```

要求至少写 15 行。

### 实验 2：最小 `no_std` 识别实验

在一个最小 `no_std` crate 中确认下面这些仍能成立：

1. `Option`
2. `Result`
3. `Cell`
4. `size_of`
5. `read_volatile`

再确认下面这些不能直接拿来就用：

1. `Vec`
2. `String`
3. `std::thread`
4. `std::fs`

重点不是把所有错误都解决，而是借这些边界报错建立认知。

---

## 今天的输出物

今天学完后，至少留下这 3 样东西：

1. 一张 `core` / `alloc` / `std` 三层边界图
2. 一张 API 归属表
3. 一段自己的总结

总结至少回答：

1. `core` 为什么能支撑很多语法，却不依赖堆和 OS？
2. `alloc` 和 `std` 分别补了什么？
3. 这和 STM32 裸机开发有什么直接关系？

---

## 今日自测题

1. 为什么 `Vec<T>` 不在 `core`？
2. 为什么 `Option<T>` 可以在 `no_std` 里正常使用？
3. `core::mem` 和 `std::mem` 的关系是什么？
4. 为什么裸机工程不依赖 `std`，但依然能使用很多“像高级语言一样”的表达？
5. `read_volatile` 为什么更应该属于 `core` 而不是 `std`？

---

## 今日自我审查

学完后，检查下面 5 件事：

1. 我是不是已经把 `core` / `alloc` / `std` 的边界说清楚了？
2. 我有没有把“Rust 标准库”模糊地全算成 `std`？
3. 我能不能举出至少 5 个 `core` API 和 5 个非 `core` API？
4. 我今天的结论是不是来自文件和实验，而不是凭感觉？
5. 我能不能把今天的认识回扣到 `stm32_rust` 工程？

---

## 完成标准

今天结束时，至少达到下面 4 条：

1. 能不用含糊词，解释 `core` / `alloc` / `std` 的区别。
2. 能正确分类常见 API。
3. 能说明为什么 `no_std` 仍然可以写出很强的抽象。
4. 能解释为什么接下来应该优先读 `core::ptr`、`core::mem`、`core::cell`。
