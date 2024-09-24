pub fn logic() {
    let x = 42;
    let r: *const i32 = &x; // 创建一个不可变原始指针
    let mut y = 0;
    let p: *mut i32 = &mut y; // 创建一个可变原始指针

    unsafe {
        *p = 10; // 使用原始指针修改值
        println!("Value of y: {}", *p); // 输出 10
    }
}

fn addr() {
    let a = 10;
    let a_ptr = &a as *const i32; // 获取 a 的内存地址（指针）
    println!("a 的地址是: {:?}", a_ptr); // 0x7f0572aa04cc
    println!("a 的地址是: {:p}", &a); // 0x7f0572aa04cc, {:p} 内部转换为 &a as *const i32
    println!("a 的值: {:?}", &a); // 10

    // 之前说到的 &a 是创建一个引用，而 {:p} 占位符则会显示该引用指向的数据在内存中的地址。
}

// test ====================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pointer_logic() {
        logic();
        addr();
    }
}
