#![allow(unused)]

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
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
