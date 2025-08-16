#![allow(unused)]
use std::fmt::{Display, Formatter};

#[derive(Debug, Default)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

#[cfg(test)]
mod default_test {
    use crate::struct_mod::Rectangle;

    #[test]
    fn default_test() {
        println!("vvv");
        println!("Rectangle::default:{:?}\n", Rectangle::default());
        println!(
            "Rectangle::default:{:?}\n",
            Rectangle {
                width: 30,
                height: 30,
            },
        );
    }
}

#[derive(Debug)]
pub struct Rectangle2 {
    pub width: u32,
    pub height: u32,
}

impl Default for Rectangle2 {
    fn default() -> Self {
        Rectangle2 {
            width: 100,
            height: 100,
        }
    }
}

#[cfg(test)]
mod default_test2 {
    use crate::struct_mod::Rectangle2;

    #[test]
    fn default_test2() {
        println!("vvv");
        println!("Rectangle::default:{:?}\n", Rectangle2::default());
    }
}
// impl Display for Rectangle {
//     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
//         return f.write_fmt(format_args!("xxx"));
//     }
// }

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
    const II: i32 = 45;

    pub fn area(&self) -> u32 {
        println!("II:{}", Rectangle::II);
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
        let rect = &Rectangle {
            width: 20,
            height: 30,
        };

        println!("{:?}", rect);
        println!("{:#?}", rect); // 换行
        println!("dbg!:{:#?}", dbg!(rect)); // 移动 打印详情
        println!("{:?}", Rectangle::square(34));
        rect.area();
    }
}

#[cfg(test)]
mod test_tuple {

    #[test]
    fn test_tuple() {
        let tp = ();
    }
}

mod test_unit_struct {
    #[derive(Debug)]
    struct UnitStruct;

    #[test]
    fn unit_struct() {
        println!("{UnitStruct:?}");
    }
}
