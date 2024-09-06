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
