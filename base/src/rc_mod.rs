// reference counting

#[cfg(test)]
mod test_rc {
    use std::rc::Rc;
    #[derive(Debug, Clone)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    // use List::{Cons, Nil};

    #[test]
    fn multi_ref() {
        // let point = Cons(1, Rc::new(Nil));
        // // let point = Rc::new(Cons(1, Rc::new(Nil)));
        // let line1 = Cons(2, Rc::clone(&point));
        // let line2 = Cons(3, Rc::clone(&point));
        //
        // println!("line1:{:?}", line1);
        // println!("line2:{:?}", line2);
    }
}
