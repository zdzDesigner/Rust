#[cfg(test)]
mod macro_test {

    macro_rules! create_fn {
        ($fn:ident) => {
            fn $fn() {
                println!("xxxxxxx");
            }
        };
    }

    #[test]
    fn test_creat() {
        create_fn!(aaa);
        aaa();
    }

    macro_rules! twice {
        ($e:expr) => {
            $e;
            $e;
        };
    }

    #[test]
    fn test_twice() {
        twice!(println!("xxx"));
    }
}

#[macro_export]
macro_rules! vvv {
    ($($x:expr), * ) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($x);)*
            temp_vec
        }
    }
}
#[cfg(test)]
mod macro_test2 {
    #[test]
    fn test_vvv() {
        let v = vvv!(4, 2, 1);
        println!("{:?}", v);
        println!("{:?}", v.len());
    }
}
