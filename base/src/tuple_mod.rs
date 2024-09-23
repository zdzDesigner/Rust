#![allow(unused)]

use std::fmt;

pub fn logic() {
    let (flag, val) = reverse((3, true));
    assert_eq!(format!("{flag}"), "true");
    assert_eq!(format!("{val}"), "3");

    diff();
}

fn reverse(pair: (u8, bool)) -> (bool, u8) {
    let (val, flag) = pair;
    return (flag, val);
}

// unit
// struct Empty();

// 元组结构体（tuple structs）
#[derive(Debug)]
struct Canvas2D(u8, &'static str);
// struct Canvas2D(u8, &str);
impl fmt::Display for Canvas2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})\n", self.0, self.1)?;
        return write!(f, "({},{})", self.1, self.0);
    }
}

fn diff() {
    let tuple = (1, true, "xxx", 2.22);
    assert_eq!(format!("{}", tuple.0), "1");
    assert_eq!(format!("{}", tuple.1), "true");
    assert_eq!(format!("{}", tuple.2), "xxx");
    assert_eq!(format!("{}", tuple.3), "2.22");

    println!("{:#?}", Canvas2D(19, "xx"));
    println!("{}", Canvas2D(19, "xx"));
}
