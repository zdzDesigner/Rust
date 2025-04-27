extern "C" {
    fn hello();
}


fn main() {
    println!("Hello, world!");
    unsafe {
        hello();
    }
}
