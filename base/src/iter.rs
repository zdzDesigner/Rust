#[cfg(test)]
mod test_iter {

    #[test]
    fn gen_iter() {
        let items = vec![2, 4, 91, 9];

        for v in items {
            println!("{:?}", v);
        }

        let items = vec!["aaa", "bbb", "cc", "ddd"];
        for v in &items { // 这里要借用
            println!("{:?}", v);
        }
        for v in items.iter() { // 这里 iter(&self) 签名中已借用
            println!("{:?}", v);
        }

        let mut iter = items.iter();
        println!("{:?}", iter.next());
        println!("{:?}", iter.next());
    }
}
