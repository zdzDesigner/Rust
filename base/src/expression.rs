pub fn express() {
    // let x = { 3 }; // {}表达式, 返回一个值
    // let x = { 3 + 1 }; // {}表达式
    let x = {
        let y = 3;
        y + 1
    }; // {}表达式

    println!("x:{}", x);

    println!("function expression:{}", ret_val());
}

fn ret_val() -> u8 {
    8
}
