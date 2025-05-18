fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

#[cfg(test)]
mod test_iter {
    // Iterator
    // FromIterator
    // IntoIterator
    use super::*;

    fn tomap(items: &[i32]) -> Vec<i32> {
        return items.iter().map(|ele| ele + 1).collect();
    }
    fn expansion(items: &[i32]) -> Vec<i32> {
        let mut items: Vec<i32> = items.iter().map(|ele| ele + 1).collect();
        items.push(4);
        items.push(4);
        items.push(4);
        items.push(4);
        items.push(4);
        items.push(4);
        return items;
    }

    #[test]
    fn gen_iter() {
        let v = 20;
        println!("&v: {:p}", &v); // 栈上 0x74c6a3ffdfbc
        let items = vec![2, 4, 91, 9];

        for v in &items {
            println!("{:?}", v);
        }
        println!("tomap: {:?}", tomap(&items.to_vec()));
        println!("tomap: {:p}", tomap(&items.to_vec()).as_ptr()); // 堆上 0x74c6a40033b0
        println!("expansion: {:?}", expansion(&items.to_vec()));
        println!("expansion: {:p}", expansion(&items.to_vec()).as_ptr()); // 堆上 0x74c6a4003500
        println!("&items:{:p}", &items); // 栈上 0x74c6a3ffe018

        let items = vec!["aaa", "bbb", "cc", "ddd"];
        println!("&items:{:p}", &items); // 栈上 0x74c6a3ffe318
        for v in &items {
            // 这里要借用
            println!("{:?}", v);
        }
        for v in items.iter() {
            // 这里 iter(&self) 签名中已借用
            println!("{:?}", v);
        }

        let mut iter = items.iter();
        println!("{:?}", iter.next());
        println!("{:?}", iter.next());
    }

    #[test]
    fn collect() {
        let arr = [2, 4, 5, 8];

        println!("arr.iter(): {:?}", arr.iter()); // Iter([2, 4, 5, 8])
        print_type_of(&arr.iter()); // core::slice::iter::Iter<i32>

        let vmap = arr.iter().map(|ele| ele + 1);
        println!("vmap: {:?}", vmap); // Map { iter: Iter([2, 4, 5, 8]) }
                                      // println!("collect: {:?}", vmap.collect());
    }
}

#[cfg(test)]
mod test_iter_peekable {
    #[test]
    fn iter_peekable() {
        let expr = "abcd";
        let iter = expr.chars().peekable();
        println!("{:?}", iter); // Peekable { iter: Chars(['a', 'b', 'c', 'd']), peeked: None }
        let v2 = iter.peekable();
        println!("{:?}", v2); // Peekable { iter: Peekable { iter: Chars(['a', 'b', 'c', 'd']), peeked: None }, peeked: None }
    }
    #[test]
    fn iter_peekable2() {
        let expr = "abcd";
        let mut iter = expr.chars().peekable();
        println!("iter:{:?}", iter);
        println!("{:?}", iter.peek()); // 存入peeked
        println!("{:?}", iter.peek());
        println!("{:?}", iter.next()); // 优先从peeked中取出
                                       // println!("{:?}", iter.next());
        println!("iter:{:?}", iter);
        println!("expr:{:?}", expr);
    }
}
