#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: Rectangle) -> u32 {
    println!("rect1 pointer:{:p}", &rectangle); // 0x7ffce4e923e8
    rectangle.width * rectangle.height
}
fn area2(rectangle: &Rectangle) -> u32 {
    println!("rect1 pointer:{:p}", rectangle); // 0x7ffce4e92470 同一指针
    rectangle.width * rectangle.height
}
fn area3(rectangle: &mut Rectangle) -> u32 {
    println!("rect1 pointer:{:p}", rectangle); // 0x7ffce4e92470 同一指针
    rectangle.width * rectangle.height
}
pub fn t_move() {
    let a = String::from("aaa");
    let b = &a;
    println!("a:{},b:{}", a, b);
}

#[cfg(test)]
mod ownership_test {
    use super::*;

    #[test]
    fn test_move_borrow_mutborrow() {
        let mut rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!("rect1:{:?}", rect1);
        println!("rect1:{:#?}", rect1);

        println!("rect1 pointer:{:p}", &rect1); // 0x7ffce4e92470

        println!(
            "The area of the rectangle is {} square pixels.",
            area3(&mut rect1)
        );
        println!(
            "The area of the rectangle is {} square pixels.",
            area2(&rect1)
        );
        println!(
            "The area of the rectangle is {} square pixels.",
            area(rect1) // move了
        );
        // println!("rect1:{:?}", rect1); 非词法作用域
    }
}
