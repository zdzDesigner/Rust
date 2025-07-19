#[derive(Debug)]
struct Line(u32);

#[derive(Debug)]
struct Box(u32, u32);
impl Box {
    fn area(&self) -> u32 {
        self.0 * self.1
    }
}

trait Multiply {
    type Output;
    fn multiply(&self, other: &Self) -> Self::Output;
}

impl Multiply for Line {
    type Output = Box;
    fn multiply(&self, other: &Self) -> Self::Output {
        Box(self.0, other.0)
    }
}

pub fn shape() {
    println!("area:{}", Line(5).multiply(&Line(4)).area());
}
