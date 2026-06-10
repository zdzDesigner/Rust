// build.rs
// 将 memory.x 放入链接器搜索路径

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    // 1. 复制 memory.x 到 OUT_DIR
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();

    // 2. 告诉链接器去哪里找脚本
    println!("cargo:rustc-link-search={}", out.display());

    // 3. 告诉链接器使用 memory.x
    println!("cargo:rustc-link-arg=-Tmemory.x");

    // 4. 变更检测
    println!("cargo:rerun-if-changed=memory.x");
}
