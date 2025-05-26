extern crate libc;

unsafe extern "C" {
    fn sum(a: libc::c_int, b: libc::c_int) -> libc::c_int;
}

fn main() {
    println!("Hello, world!");
    let a = 4;
    let b = 5;
    unsafe { println!("{:?}", sum(a, b)) };
}
