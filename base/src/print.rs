// use std::fmt::{self, Display, Formatter};
use std::fmt;

pub fn print() {
    base();
    print_structure();
    print_city();
}

struct City {
    // name: &'static str, // 静态区
    name: String, // 堆区
    // 纬度
    lat: f32,
    // 经度
    lon: f32,
}

impl fmt::Display for City {
    // `f` 是一个缓冲区（buffer），此方法必须将格式化后的字符串写入其中
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` 和 `format!` 类似，但它会将格式化后的字符串写入
        // 一个缓冲区（即第一个参数f）中。
        return write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        );
    }
}

fn print_city() {
    for display in [City {
        // name: "xx",
        name: String::from("xx"),
        lat: 20.2,
        lon: 30.3,
    }] {
        println!("{}", display);
    }
}

// #[warn(dead_code)]
#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
fn print_structure() {
    for color in [
        Color {
            red: 200,
            green: 210,
            blue: 220,
        },
        Color {
            red: 100,
            green: 110,
            blue: 120,
        },
    ]
    .iter()
    {
        println!("{:?}", color);
    }
}

fn base() {
    println!("Hello, world!");

    let x = 3 + /*xx*/ 5;
    println!("x = {x}");
    // 8

    // 类型 ================
    println!("{}", 31i64);
    // 31  i64

    // 下标 {int} ===============
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // Alice, this is Bob. Bob, this is Alice

    // 命名 {name} ==============
    println!("{one} {two}", one = "one message", two = "two");
    // one message two

    // 进制 ":b" ================
    println!("{:b} this is :b", 6);
    // :b to binary  "110 this is :b"

    // 对齐 ":<" ":>" ":^" ======
    println!("{num:>width$}前面2位空格", num = 3, width = 2);
    // 3前面2位空格

    // 左对齐
    assert_eq!(format!("Hello {:<5}!", "x"), "Hello x    !");
    // Hello x    !

    // 左对齐, 填充-
    assert_eq!(format!("Hello {:-<5}!", "x"), "Hello x----!");
    // Hello x----!

    // 居中
    assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");
    // Hello   x  !

    // 右对齐
    assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");
    // Hello     x!

    // 小数 ":." ================
    println!("{:.2}", 3.14159);

    #[derive(Debug)]
    #[allow(dead_code)]
    struct V(i32, u8);

    // Debug ":?" ===============
    println!("{:?}", V(3, 2));
    println!("{:#?}", V(3, 2));

    // std::fmt::Display(); // {}
    // std::fmt::Debug(); // {:?}
}
