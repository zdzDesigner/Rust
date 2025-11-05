// 声明方式: 项目中的模块声明方式
// 1. 外层文件(utils.rs)和文件夹(utils)名称相同
// 2. 在文件夹中存在 mod.rs (components/mod.rs)

// 引用方式
// 1. 通过use 依赖 `lib.rs` 或 `外部` 库
// 2. 通过 mod 直接依赖项目中的模块

use bus::getlib; 
use bus::utils::tool::get_tool;
use bus::Input;

mod components;
mod utils;

fn main() {
    println!("Hello, world!");
    println!("{}", getlib());
    println!("{}", get_tool());
    println!("{:?}", Input {});

    println!("from mod:{}", utils::tool::get_tool());
    components::input::Input {};
    components::radio::Radio {};
}
