pub fn err_mut() {
    let mut v = String::from("zdz");

    let v1 = &mut v;
    _ = v1;
    let v2 = &mut v;
    _ = v2;

    // 出错出现交集(memory intersection)
    // let v1 = &mut v;
    // let v2 = &mut v;
    // _ = v1;
    // _ = v2;
}
