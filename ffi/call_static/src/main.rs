include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

unsafe extern "C" {
    fn sum(a: libc::c_int, b: libc::c_int) -> libc::c_int;
    fn div(a: libc::c_int, b: libc::c_int) -> libc::c_int;
}

fn main() {
    println!("Hello, world!");
    unsafe {
        let res = sum(3, 4);
        println!("{res}");
        let res_div = div(20, 1);
        println!("{res_div}");

        let res_multi = multiplication(4, 2);
        println!("{res_multi}");
    }
}
