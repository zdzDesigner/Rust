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

#[cfg(test)]
mod test_custom_box {
    use std::ops::Deref;

    #[derive(Debug)]
    struct Box<T>(T);

    impl<T> Box<T> {
        fn new(t: T) -> Box<T> {
            Box(t)
        }
        // fn into(self) -> T { // move
        //     self.0
        // }
    }
    impl<T> Deref for Box<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[test]
    fn base() {
        let b = Box::new(3);
        println!("{:?}", b);
        println!("{}", b.deref());
        println!("{}", *b.deref());
        // println!("{}", b.into()); // move
    }
}
