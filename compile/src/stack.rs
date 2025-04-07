// MIR
// fn main() -> () {
//     let mut _0: ();
//     let _1: (); // ()单元类型
//
//     bb0: {
//         // 这里的continue并非传统循环中的继续，而是出现错误时跳过当前基本块（`bb0`）的剩余代码，直接进入清理阶段
//         // 这是 MIR 中的**控制流标注**，表示两种可能的执行路径：
//         // • **正常返回**：跳转到 `bb1`（执行 `return`）。
//         // • **栈展开**：执行 `unwind continue`。
//         _1 = basetype() -> [return: bb1, unwind continue];
//     }
//
//     bb1: {
//         return;
//     }
// }

// 返回值: ()单元类型
pub fn basetype() {
    let x = 5;
    let y = 10;
    let z = if x > y { x + y } else { x * y };
    _ = z;
}
