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

        eprintln!("rect1:{:?}", rect1);
        println!("rect1:{:#?}", rect1);

        println!("rect1 pointer:{:p}", &rect1); // 0x7ffce4e92470

        println!(
            "The area of the rectangle is {} square pixels.",
            area3(&mut rect1)
        );
        eprintln!("&mut rect1:{:?}", rect1);
        println!(
            "The area of the rectangle is {} square pixels.",
            area2(&rect1)
        );
        eprintln!("&rect1:{:?}", rect1);
        println!(
            "The area of the rectangle is {} square pixels.",
            area(rect1) // 丢失所有权(move了), 后续无法访问
        );
        // println!("rect1:{:?}", rect1); 非词法作用域
    }
}

#[cfg(test)]
mod ownership_move_test {
    // use super::*;

    #[derive(Debug)]
    struct V {
        name: String,
    }

    impl Drop for V {
        // 自定义drop (不会覆盖默认drop), 只是查看调用过程
        fn drop(&mut self) {
            println!("drop ==========");
        }
    }
    fn ref_handler(v: &V) {
        // 指针引用
        println!("{:?}", v);
        println!("{:p}", v);
    }
    fn ref_mut_handler(v: &mut V) {
        v.name.push_str("abcdefghijkaaavvvvvvvvvvvvvvvvvvvvvvv");
    }

    fn move_2_handler(v: String) {
        println!("{:?}", v);
    }

    #[test]
    fn move_1() {
        let v1 = String::from("zdz");
        // let v2 = v1; //  move
        println!("{:?}", v1);
    }

    #[test]
    fn move_2() {
        let v1 = String::from("zdz");
        move_2_handler(v1);
        // println!("{}", v1); // move
    }

    // 悬垂指针
    // this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    // fn dangling() -> &String {
    //     let v1 = String::from("zdz");
    //     return &v1;
    // }
    fn create_str() -> String {
        let v1 = String::from("zdz");
        return v1;
    }

    #[test]
    fn move_3() {
        let v1 = create_str();
        println!("{}", v1);
    }

    #[test]
    fn test_move() {
        let arr = vec![2, 3, 4, 5];
        let mut v = V {
            name: String::from("aa"),
        };
        println!("{:p}", &v);
        println!("{:p}", &v.name);
        ref_handler(&v);
        for i in 0..10 {
            ref_mut_handler(&mut v);
            println!("{:p}", v.name.as_ptr()); // 增长一定长度时指针变化
                                               // if let ret = v.name.get(0..) {
                                               //     println!("{}", ret);
                                               // }
        }
        let ret = match v.name.get(v.name.len() - 10..) {
            Some(val) => val,
            None => "NotFound",
        };
        {
            let v = V {
                name: String::from("aa"),
            };
            println!("block drop pre =====");
        }
        println!("== drop pre==== {} =======", ret);
    }
}
