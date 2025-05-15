pub fn cc() {
    println!("cc");
}

mod a {
    pub fn foo() {
        println!("foo");
    }

    pub mod b {
        pub mod c {
            pub fn foo() {
                super::super::foo(); // 调用 a'的 foo 函数
                self::super::super::foo(); // 调用 a'的 foo 函数
            }
        }
    }
}

mod b {
    fn bar() {}
}

// fn main() {
//     // cc();
//     a::b::c::foo();
// }
