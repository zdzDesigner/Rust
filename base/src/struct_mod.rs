#![allow(unused)]
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        return f.write_fmt(format_args!("xxx"));
    }
}

impl Rectangle {
    // 关联函数
    pub fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 可以多个块
impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
    pub fn area1(self: &Self) -> u32 {
        self.width * self.height
    }
}

#[cfg(test)]
mod test_struct_tuple {

    #[derive(Debug)]
    struct Message(String, i32);

    #[test]
    fn test_struct_tuple() {
        let message = Message(String::from("warning"), 888);
        println!("{:?}", message.0);
        println!("{:?}", message.1);
    }
}
mod test_struct_fmt {
    use super::*;

    #[test]
    fn test_println() {
        let rect = Rectangle {
            width: 20,
            height: 30,
        };

        println!("{:?}", rect);
        println!("{:#?}", rect); // 换行
        println!("dbg!:{:#?}", dbg!(rect)); // 打印详情
    }
}

#[cfg(test)]
mod test_tuple {

    #[test]
    fn test_tuple() {
        let tp = ();
    }
}
