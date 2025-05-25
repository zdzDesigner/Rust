use cc;
use std::{env, path::PathBuf};

fn main() {
    // ===========================
    // 指定静态库的搜索路径, 查找静态库
    // ===========================
    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-lib=static=sum");

    // ===========================
    // 查找动态库 ================
    // ===========================
    println!("cargo:rustc-link-search=native=lib/shared");
    println!("cargo:rustc-link-lib=dylib=div");
    println!("cargo:rustc-link-arg=-Wl,-rpath,lib/shared"); // 添加运行时参数, -rpath动态库路径

    // ===========================
    // 自动导入 ==================
    // ===========================
    // 直接编译
    cc::Build::new()
        .file("libgen/multiplication.c")
        .compile("libmultiplication.a");

    // 导入头文件(单个) ==========
    // let header = "./libgen/multiplication.h";
    // let bindings = bindgen::Builder::default()
    //     .header(header)
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    //     .generate()
    //     .expect("generate bindings error!");

    // 导入头文件(多个) ==========
    let headers = vec!["./libgen/multiplication.h"];
    let mut builder = bindgen::Builder::default()
        // .clang_arg("-Ilibgen/") // ??!!(无效) 添加头文件搜索路径
        .allowlist_recursively(false);

    // 添加所有头文件
    for h in headers {
        builder = builder.header(h);
    }
    let bindings = builder.generate().expect("无法生成绑定");

    let outpath = PathBuf::from(env::var("OUT_DIR").unwrap());
    // ./target/debug/build/call_static-03a1b64bdfc48318/out/bindings.rs

    bindings
        .write_to_file(outpath.join("bindings.rs"))
        .expect("write bindings error!");
}
