# 指针、裸指针与 MMIO 访问链路

这篇文档专门解决一个在 Rust 嵌入式学习里最容易混淆的问题：

1. 什么是引用
2. 什么是裸指针
3. 为什么 `*const T` 前面要强调“裸”
4. 为什么寄存器访问代码里经常出现 `&*ptr`
5. `UnsafeCell`、`VolatileCell`、`read_volatile` / `write_volatile` 分别在解决什么问题

这篇文档建议和下面这些文件一起看：

1. `/home/zdz/Documents/Try/Rust/learning/stm32_rust/courses/core/day01.md`
2. `/home/zdz/mnt/Documents/Try/Rust/learning/stm32_rust/src/volatile.rs`
3. `/home/zdz/mnt/Documents/Try/Rust/learning/stm32_rust/src/rcc.rs`
4. `/home/zdz/mnt/Documents/Try/Rust/learning/stm32_rust/src/gpio.rs`

---

## 一、先把“指针”分成两类

在 Rust 里，如果你只说“指针”，范围太大了。

因为下面这些都和“指向某块内存”有关：

1. `&T`
2. `&mut T`
3. `*const T`
4. `*mut T`
5. `Box<T>`、`Rc<T>` 这类智能指针

所以 Rust 语境里必须进一步细分。

最重要的两类是：

1. **引用**：`&T`、`&mut T`
2. **裸指针**：`*const T`、`*mut T`

---

## 二、引用是什么

例如：

```rust
let x = 10;
let r = &x;
```

这里：

1. `x` 的类型是 `i32`
2. `r` 的类型是 `&i32`

`r` 当然也指向某个地址，所以广义上它也是一种“指针”。

但 Rust 不把它叫“裸指针”，而是叫“引用”，因为它带着一整套额外语义：

1. 受借用规则约束
2. 默认要求指向有效内存
3. 默认要求满足对齐
4. 通常不能是 null
5. 编译器可以基于这些承诺做优化

所以引用不是“普通地址”，而是“带约束、带承诺的安全指针”。

---

## 三、裸指针是什么

例如：

```rust
let p: *const i32 = &x as *const i32;
```

这里：

1. `p` 的类型是 `*const i32`
2. 这就是裸指针

裸指针的特点是：

1. 不受借用检查保护
2. 可以是无效地址
3. 可以是 null
4. 可以未对齐
5. 解引用必须放在 `unsafe` 里

所以裸指针不是“更底层版本的引用”那么简单，而是“只有地址语义，没有安全承诺的原始指针”。

---

## 四、为什么前面要加“裸”字

因为在 Rust 里，“指针”这个词本身不够精确。

如果只说“指针”，别人不知道你说的是：

1. 引用 `&T`
2. 裸指针 `*const T`
3. 智能指针 `Box<T>`

“裸指针”这个说法是在强调：

1. 它是最原始的地址形式
2. 它没有引用那样的借用语义
3. 它没有有效性、非空、对齐这些默认保证
4. 编译器不会替你兜底

可以粗暴理解成：

1. 引用 = 被 Rust 安全规则包起来的指针
2. 裸指针 = 把这层包装剥掉以后，剩下的地址本体

所以这里的“裸”，不是装饰词，而是在提醒你：

1. 这里只有地址
2. 没有保护层
3. 这是低层能力

---

## 五、引用和裸指针的对照表

| 类型 | 是不是指针 | 是否受借用规则约束 | 是否可直接安全解引用 |
| :--- | :--- | :--- | :--- |
| `&T` | 是 | 是 | 是 |
| `&mut T` | 是 | 是 | 是 |
| `*const T` | 是 | 否 | 否 |
| `*mut T` | 是 | 否 | 否 |

所以一定要记住：

1. **引用也是指针**
2. **裸指针也是指针**
3. 它们最大的区别不是“长得不一样”，而是“安全模型不一样”

---

## 五点五、和 Zig 的第一层对比

如果你有 Zig 背景，这里最容易误解的一点是：

1. Rust 的 `&T` 不是 Zig 普通指针的直接等价物。
2. Rust 的 `*const T` / `*mut T` 才更接近“原始地址视图”这个层次。

但即使这样，两边也不能简单一一对号。

### 1. Rust 的引用 vs Zig 的指针

Rust：

1. `&T` 带借用语义、有效性假设、对齐假设。
2. `&mut T` 还额外带独占访问语义。

Zig：

1. `*T` / `*const T` 主要表达“这是一个指向某类型的地址”。
2. Zig 没有 Rust 这种借用检查器，也没有 `&T` / `&mut T` 这套共享/独占引用语义。

所以对照时更准确的理解是：

1. Rust 的 `&T` 更像“带语言级别约束的安全访问视图”。
2. Zig 的 `*T` / `*const T` 更像“显式指针访问模型”。

### 2. Rust 的裸指针 vs Zig 的普通指针

Rust 的 `*const T` / `*mut T` 和 Zig 的普通指针更接近，但也不能简单说“完全一样”。

共同点：

1. 都更接近原始地址语义。
2. 都适合表达寄存器块基地址、缓冲区地址、FFI 边界。

差异点：

1. Rust 会更明确地区分“引用”和“裸指针”两套语义世界。
2. Zig 更强调显式指针种类本身，例如 `*T`、`[*]T`、`[*c]T`、`?*T` 等。

所以你在 Zig 里觉得“很自然就是指针”的地方，到了 Rust 里往往要先问一句：

1. 这里到底该是引用，还是裸指针？
2. 如果是引用，我是否真的满足 Rust 对引用的语义承诺？

---

## 六、放到当前项目里看

在这个项目里，你会看到类似代码：

```rust
pub struct Rcc {
    regs: *const Regs,
}
```

这里：

1. `regs` 是裸指针
2. 类型是 `*const Regs`
3. 它表示“RCC 寄存器块基地址”

例如：

```rust
const RCC_BASE: *const Regs = 0x4002_1000 as *const Regs;
```

这说明：

1. 我手里有一个固定硬件地址
2. 我把它当成 `Regs` 结构体来看
3. 但这只是我自己的承诺，编译器不会自动相信它安全

所以这一步天然适合先用裸指针表达，而不是一上来就用引用。

---

## 七、`let regs = unsafe { &*self.regs };` 到底在做什么

这是当前代码里非常关键的一句。

```rust
let regs = unsafe { &*self.regs };
```

假设：

```rust
self.regs: *const Regs
```

那么类型变化是这样的：

1. `self.regs`
   - 类型：`*const Regs`
   - 含义：裸指针

2. `*self.regs`
   - 类型：`Regs`
   - 含义：解引用裸指针，访问它指向的那块内存
   - 这是 `unsafe`，因为编译器不能证明这个地址有效

3. `&*self.regs`
   - 类型：`&Regs`
   - 含义：把这块内存借成一个共享引用，方便字段访问

所以最终：

```rust
let regs = unsafe { &*self.regs };
```

得到的 `regs` 是：

```rust
&Regs
```

不是裸指针。

这句的真实意思是：

1. 我手里有一个寄存器块基地址的裸指针
2. 我现在临时把它提升成 `&Regs` 视图
3. 这样我就能方便地写 `regs.apb2enr.get()` 这种字段访问

---

## 八、为什么不一直用裸指针

因为裸指针虽然底层，但不方便表达结构化访问。

如果只保留裸指针，你后面会写成：

```rust
unsafe { (*self.regs).apb2enr.get() }
```

这当然能工作，但可读性更差。

把它先转成：

```rust
let regs = unsafe { &*self.regs };
```

之后就能自然写：

```rust
let current = regs.apb2enr.get();
```

所以这个转换的目的不是“把指针又变回指针”，而是：

1. 从裸地址
2. 变成可字段访问的结构体视图

---

## 九、这里为什么还需要 `VolatileCell`

即使你已经拿到了：

```rust
&Regs
```

也不能直接把寄存器字段写成普通 `u32`。

例如下面这种写法就不对：

```rust
#[repr(C)]
struct Regs {
    apb2enr: u32,
}
```

原因是：

1. 这会变成普通内存读写语义
2. 编译器可能优化掉访问
3. 不符合 MMIO 的真实访问要求

所以项目里用的是：

```rust
apb2enr: VolatileCell<u32>
```

然后这样访问：

```rust
regs.apb2enr.get()
regs.apb2enr.set(...)
```

这样读写会走 volatile 语义。

---

## 十、`UnsafeCell` 在这里解决什么问题

`VolatileCell<T>` 内部是这样实现的：

```rust
pub struct VolatileCell<T> {
    value: UnsafeCell<T>,
}
```

这不是多余的。

因为你当前拿到的通常是：

```rust
&Regs
```

按普通 Rust 语义，`&T` 表示共享引用，编译器会默认认为：

1. 这块内存不会通过共享引用被随意改写
2. 可以基于这个假设做优化

但硬件寄存器不是普通内存：

1. 它可能被中断访问
2. 它可能被硬件自己改变
3. 它经常需要通过共享引用路径进行读写

所以 `UnsafeCell<T>` 的作用是：

1. 打破 `&T` 的普通不可变假设
2. 告诉编译器：这块内存的可变性规则不是普通 Rust 数据那一套

要点是：

1. `UnsafeCell` 解决的是“共享引用下允许底层可变”
2. `read_volatile` / `write_volatile` 解决的是“这次访问必须真实发生”

这两者不是一回事，不能混为一谈。

---

## 十一、MMIO 访问链路总图

把当前项目的访问路径串起来，就是：

```text
固定硬件地址
-> 裸指针 *const Regs
-> unsafe { &*ptr } 得到 &Regs
-> 字段类型是 VolatileCell<u32>
-> VolatileCell 内部持有 UnsafeCell<T>
-> get/set 最终调用 read_volatile / write_volatile
```

也可以写成更直观的分层图：

```text
RCC_BASE / GPIOC_BASE
  |
  +-- *const Regs
  |     裸指针，只表示“这是寄存器块地址”
  |
  +-- unsafe { &*ptr }
  |     临时转成 &Regs
  |     方便写 regs.apb2enr / regs.crh / regs.bsrr
  |
  +-- regs.apb2enr
  |     字段类型是 VolatileCell<u32>
  |
  +-- VolatileCell<T>
  |     内部包着 UnsafeCell<T>
  |
  +-- UnsafeCell<T>
  |     允许通过共享引用访问可变底层内存
  |
  +-- read_volatile / write_volatile
        真正完成 MMIO 读写
```

---

## 十一点五、和 Zig 的第二层对比：MMIO 怎么落地

如果把当前项目里的 Rust MMIO 链路和 Zig 对照，最核心的差异不是“谁能不能访问寄存器”，而是“语言要求你如何表达这件事”。

### 1. Zig 常见思路

在 Zig 里，你更容易写出类似这种路径：

```text
固定地址
-> 指针类型转换
-> `volatile` 指针或带 volatile 语义的访问
-> 直接读写寄存器字段
```

它强调的是：

1. 地址是什么
2. 这个地址按什么类型解释
3. 访问时是否带 `volatile` 语义

### 2. Rust 当前项目的思路

在这个项目里，Rust 的路径是：

```text
固定地址
-> 裸指针 `*const Regs`
-> `unsafe { &*ptr }` 得到结构体视图 `&Regs`
-> 字段是 `VolatileCell<u32>`
-> `VolatileCell` 内部持有 `UnsafeCell<T>`
-> `get/set` 最终调用 `read_volatile` / `write_volatile`
```

这里多出来的层次，主要是在显式表达两件 Zig 里不需要以同样方式表达的事：

1. 共享引用下的底层可变性问题
2. Rust 对引用语义的严格约束

### 3. 最容易误解的点

最容易误解的一句话是：

1. “Zig 里直接指针访问就行，Rust 为什么这么绕？”

更准确的说法应该是：

1. Zig 侧重显式指针和访问方式本身。
2. Rust 不只在问“能不能访问”，还在问“你是不是在对编译器做某种引用语义承诺”。

所以 Rust 这里的 `UnsafeCell` 不是“多余包装”，而是在修正普通 `&T` 不适用于硬件寄存器的那部分假设。

### 4. 对你当前项目最有价值的 Zig 对照结论

如果你从 Zig 切到这个项目里的 Rust，最有价值的对照不是 API 逐个翻译，而是记住下面 4 条：

1. Zig 里“这是一个指针”很多时候已经足够表达意图；Rust 里你要先区分“这是引用还是裸指针”。
2. Zig 里你更常先想访问路径；Rust 里你还要额外想这条路径是否违反了引用语义。
3. Zig 里 `volatile` 主要解决访问语义；Rust 里除了 volatile 访问语义，还要单独处理 `UnsafeCell` 这层内部可变性。
4. Zig 的 `packed struct`、指针类型系统、`volatile` 访问模型都很强，但不能直接套用为 Rust 的 `repr(C)`、引用、裸指针和 `VolatileCell`。

---

## 十二、一定要分清这 4 个概念

后面学习 `core::ptr`、`core::cell`、HAL 寄存器封装时，最容易把下面 4 个问题混成一个：

1. `&T`
   - 引用
   - 是安全语义包装过的指针

2. `*const T` / `*mut T`
   - 裸指针
   - 只有地址，没有安全承诺

3. `UnsafeCell<T>`
   - 解决共享引用下的底层可变性问题

4. `read_volatile` / `write_volatile`
   - 解决 MMIO 访问不能被优化掉的问题

这 4 个东西分别解决不同问题，不能混成一句“都是为了安全访问寄存器”。

---

## 十三、放回 Day 1 主线里怎么理解

这篇文档和当前课程主线的关系是：

1. `src/` 里写 HAL 时，寄存器块基地址通常先用裸指针保存。
2. 在 HAL 内部访问寄存器时，再通过 `&*ptr` 临时得到结构体视图。
3. 字段本身不能是普通 `u32`，而应该通过 `VolatileCell` 之类的封装承载 MMIO 语义。
4. `examples/` 里不应该重复实现这套底层逻辑，而应该消费 `src/` 里的 HAL。

所以这篇文档的重点不是“学会一种奇怪语法”，而是理解为什么项目结构要这样分层。

---

## 十四、自测题

1. 为什么说引用也是指针，但 Rust 里不把它叫“裸指针”？
2. `*const T` 和 `&T` 的本质区别是什么？
3. `let regs = unsafe { &*self.regs };` 得到的最终类型是什么？
4. 为什么寄存器字段不能直接写成普通 `u32`？
5. `UnsafeCell<T>` 和 `read_volatile` / `write_volatile` 分别解决什么问题？
6. 为什么说“裸指针 -> 结构体视图 -> VolatileCell -> volatile 读写”是一条完整链路？

---

## 十五、收尾结论

一句话总结这篇文档：

1. **引用也是指针，但它是带借用语义和有效性承诺的指针。**
2. **裸指针是只有地址、没有安全承诺的原始指针。**
3. **MMIO 寄存器访问不是只靠一种机制完成的，而是裸指针、引用视图、`UnsafeCell`、volatile 访问语义共同配合的结果。**
4. **如果用 Zig 作对照，最重要的不是把语法一一翻译，而是理解 Rust 为什么要额外区分“引用语义”和“原始地址语义”。**
