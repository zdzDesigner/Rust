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


    // use std::rc::Rc;

    #[test]
    fn multi_ref() {
        let point = Cons(1, Box::new(Nil));
        let line1 = Cons(2, Box::new(point));
        // let line2 = Cons(3, Box::new(point)); //[E0382] use of moved value: `point`value used here after move
        // to rc_mod.rs

        // let point = Cons(1, Rc::new(Nil));
        // let line1 = Cons(2, Rc::new(point));
        // let line2 = Cons(3, Rc::new(point));
        //
        // println!("line1:{:?}", line1);
        // println!("line2:{:?}", line2);
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
        assert_eq!(3, *b); // Deref
                           // println!("{}", b.into()); // move
    }

    // 隐士解引用
    fn force_conv(name: &str) {
        println!("name:{}", name);
    }
    #[test]
    fn test_force_conv() {
        force_conv(&Box::new(String::from("zdz")));
        force_conv(&String::from("zdz"));
    }
}

#[cfg(test)]
mod test_deref {
    use std::ops::Deref;

    struct Box<T> {
        item: T,
    }

    impl<T> Deref for Box<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.item
        }
    }

    #[test]
    fn base() {
        let v = Box { item: 3 };
        assert_eq!(3, *v);
    }
}

#[cfg(test)]
mod test_drop {
    struct User {
        name: String,
    }

    impl Drop for User {
        fn drop(&mut self) {
            println!("user is drop:{}!", self.name);
        }
    }

    #[test]
    fn do_drop() {
        let user = User {
            name: String::from("zdz"),
        };

        {
            let user = User {
                name: String::from("zym"),
            };
        }

        // 顺序被丢弃
        println!(" drop ==========");
    }

    #[test]
    fn force_drop() {
        let user = User {
            name: String::from("zdz"),
        };
        drop(user); // 强制丢弃
                    // drop(user); // !!=> 编译器检测
        println!("强制丢弃!! ========");
        {
            let user = User {
                name: String::from("zym"),
            };
        }

        println!(" drop ==========");
    }
}
