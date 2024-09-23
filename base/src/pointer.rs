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

// test ====================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pointer_logic() {
        logic();
    }
}
