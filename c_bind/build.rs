fn main() {
    println!("=== 构建环境 ===");

    if let Ok(v) = std::env::var("CARGO_QUIET") {
        println!("v:{v}");
    } else {
        println!("=== NOT SET CARGO_QUIET ===");
    }
    // panic!("xxxxxx");
    cc::Build::new().file("lib/hello.c").compile("hello");

    println!("cargo:rustc-link-lib=stdio"); // 把这里写入到编译参数中
}
