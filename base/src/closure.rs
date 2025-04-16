#[cfg(test)]
mod test_closure {
    #[test] // 泛型类型推导
    fn test_closure_type_derivation() {
        let derivation = |num| -> u32 { num };
        println!("type derivation: {:?}", derivation(8));
    }

    #[test]
    fn test_closure_base() {
        let tofn = |num: u8| {
            println!("inner closure: {:?}", num);
        };
        tofn(8);
    }

    #[test]
    fn test_closure_scope() {
        let name = "zdz";
        let canuse_fn_scope = || {
            println!("name:{:?}", name);
        };
        canuse_fn_scope();
    }
}

#[cfg(test)]
mod test_closure_cache {
    // use std::fmt::{Display, Formatter};

    #[derive(Debug)]
    struct Cache<T, U>
    where
        T: Copy,
        U: Fn(T) -> T,
    {
        closure: U,
        val: Option<T>,
    }

    // impl Display for Cache<T, U> {
    //     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    //         return f.write_fmt(format_args!("xxx"));
    //     }
    // }

    impl<T, U> Cache<T, U>
    where
        T: Copy,
        U: Fn(T) -> T,
    {
        fn new(closure: U) -> Cache<T, U> {
            Cache { closure, val: None }
        }

        fn value(&mut self, v: T) -> T {
            match self.val {
                Some(res) => res,
                None => {
                    let val = (self.closure)(v); // 使用 `(self.closure)` 包裹
                    self.val = Some(val);
                    val
                }
            }
        }
    }
    #[test]
    fn test_cache() {
        // 是用`_`代替闭包类型
        let mut cache = Cache::<u32, _>::new(|num| num);

        let val = cache.value(30);
        println!("res:{:?}", val);
        println!("cache val: {:?}", cache.val);

        // println!("cache: {:?}", cache);
    }
}

#[cfg(test)]
mod test_closure_move {

    #[test]
    fn closure_move() {
        let nums = vec![3, 1, 8];

        let fnclosure = move |x: Vec<i32>| x == nums; // 显式move

        // println!("{:?}", nums);
    }
}
