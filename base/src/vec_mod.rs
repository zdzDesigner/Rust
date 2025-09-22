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

#[cfg(test)]
mod test_vec {

    #[test]
    fn parse_reference() {
        let a = [1, 3, 4];

        for item in &a {
            println!("{item}");
        }
    }

    #[test]
    fn test_vec_get() {
        let mut xpoints: Vec<i32> = Vec::new();
        println!("xpoints:{:?}", xpoints);
        println!("xpoints length:{:?}", xpoints.len());

        xpoints.push(3);
        println!("xpoints:{:?}", xpoints);
        println!("xpoints length:{:?}", xpoints.len());

        println!("vec get:{:?}", xpoints.get(0));
        println!("vec get:{:?}", xpoints.get(1));
    }

    #[test]
    fn test_borrow() {
        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];
        v.push(6); // 重新分配内存, first 已被释放

        // println!("The first element is: {:?}", first);
        // Error

        // let first = &v[0]; // 可以使用let重新赋值, 后续继续访问
        // println!("The first element is: {:?}", first);
        // let v: Vec<i32> = v.iter().map(|v| v + 3).collect();
        // collect 实现FromIterator trait 的类型
        v.iter().map(|v| v + 3).collect::<Vec<i32>>();
    }

    #[derive(Debug)]
    enum SqlOption {
        Int(i32),
        Text(String),
    }

    #[test]
    fn difftype_vec() {
        let row = vec![SqlOption::Int(22), SqlOption::Text(String::from("blob"))];

        println!("row:{:?}", row); // row:[Int(22), Text("xxxxx")]
        println!("row length:{}", row.len()) // 2
    }
}

#[cfg(test)]
mod test_vec_api {
    #[test]
    fn test_vec() {
        let list = vec![23, 32, 2];
        let map = list.iter().map(|v| v + 1);
        println!("{:?}", map);
        map.map(|v| v + 1).into_iter().collect::<Vec<i32>>();
    }
}

mod test_array {
    #[test]
    fn test_array() {
        let ys: [i32; 500] = [0; 500];
        println!("array:{}", ys.len());

        println!("len:{}", ys[0..10].len());
        println!("len:{}", &ys[0..10].len());
    }

    #[test]
    fn array_to_vec() {
        let v = [2, 39, 1];

        assert_eq!(v, *(v.to_vec()));
    }

    #[test]
    fn multi_array() {
        // 编译器自动推导出one的类型
        let one = [1, 2, 3];
        // 显式类型标注
        let two: [u8; 3] = [1, 2, 3];
        let blank1 = [0; 3];
        let blank2: [u8; 3] = [0; 3];

        // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
        let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];

        // 借用arrays的元素用作循环中
        for a in &arrays {
            print!("{:?}: ", a);
            // 将a变成一个迭代器，用于循环
            // 你也可以直接用for n in a {}来进行循环
            for n in a.iter() {
                print!("\t{} + 10 = {}", n, n + 10);
            }

            let mut sum = 0;
            // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
            for i in 0..a.len() {
                sum += a[i];
            }
            println!("\t({:?} = {})", a, sum);
        }
    }
}
