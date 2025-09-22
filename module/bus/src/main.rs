use bus::getlib;
use bus::utils::tool::get_tool;
use bus::Input;

fn main() {
    println!("Hello, world!");
    println!("{}", getlib());
    println!("{}", get_tool());
    println!("{:?}", Input {});
}
