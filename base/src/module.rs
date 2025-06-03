pub fn cc() {
    println!("cc");
}

mod a {
    pub use b::c::bar;

    pub fn foo() {
        println!("foo");
    }

    pub mod b {
        pub mod c {
            pub fn foo() {
                // super::super::foo(); // 调用 a'的 foo 函数
                self::super::super::foo(); // 调用 a'的 foo 函数
            }
            pub fn bar() {
                println!("this is bar");
            }
        }
    }
}

mod b {
    pub fn bar() {
        println!("this is b bar");
    }
}

#[cfg(test)]
mod test_module {
    use super::*;
    #[test]
    fn use_module() {
        a::foo();
        a::bar();
        b::bar();
    }
}
