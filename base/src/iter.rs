#[cfg(test)]
mod test_iter {
    // Iterator
    // FromIterator
    // IntoIterator

    #[test]
    fn gen_iter() {
        let items = vec![2, 4, 91, 9];

        for v in items {
            println!("{:?}", v);
        }

        let items = vec!["aaa", "bbb", "cc", "ddd"];
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
    fn collect() {}
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
        println!("expr:{:?}",expr);
    }
}
