#[cfg(test)]
mod test_box {

    #[test]
    fn box_base() {
        let v = Box::new(9);
        println!("v:{}", v);
        println!("heap:{:p}", v);
        println!("stack:{:p}", &v);
    }
}

#[cfg(test)]
mod test_cons_list {
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    #[test]
    fn cons_list() {
        let link = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        println!("{:?}", link);
    }
}
