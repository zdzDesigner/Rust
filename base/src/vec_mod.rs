#![allow(unused)]

pub fn vec_logic() {
    let mut v: Vec<u16> = Vec::new();
    println!("{:?}", v);
    println!("{}", v.len());

    v.push(8);
    println!("{:?}", v);

    // ===================================
    let mut v2 = vec![1, 3, 4];
    println!("{:?}", v2);
    println!("v2[0]:{}", v2[0]);

    let item = match v2.get(0) {
        // get(0): Option<&T>
        Some(v) => v,
        None => &0,
    };

    println!("item:{}", item);
    println!("v2[0] == v2.get(0) Some:{}", *item == v2[0]);
    println!("v2[0] == v2.get(0) Some:{}", item == &v2[0]);

    let first = &v2[0];

    v2.push(5);
    println!("v2:{:?}", v2);
}

pub fn t_borrow() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6); // 重新分配内存, first 已被释放

    // println!("The first element is: {:?}", first);
    // Error
}

pub fn for_vec() {
    let mut v = vec![1, 3, 4, 5];
    println!("v:{:?}", v);

    // 这里是&v, 直接使用 v 则是move
    for i in &v {
        println!("item:{}", i);
    }
    println!("v:{:?}", v);

    for i in &mut v {
        *i += 10;
    }
    println!("v:{:?}", v);

    // v.iter().zip(other).collect()
    for i in v.iter() {
        println!("iter:{}", i);
    }
}

#[derive(Debug)]
enum SqlOption {
    Int(i32),
    Text(String),
}

pub fn difftype_vec() {
    let row = vec![SqlOption::Int(22), SqlOption::Text(String::from("xxxxx"))];

    println!("row:{:?}", row);
}
