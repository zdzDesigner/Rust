// mod print;
// mod tuple;
// mod typebase;
// mod ownership;
mod enum_mod;
mod error_mod;
mod hashmap_mod;
mod module_mod;
mod string_mod;
mod struct_mod;
mod vec_mod;
mod std_mod {
    pub mod fs {
        pub mod open;
    }
}

use module_mod::back_of_house;
use std_mod::fs::open as fs;

fn main() {
    // print::print();
    // tuple::logic();
    // typebase::logic();
    // ownership::ownership();

    // struct_mod ====================================
    let rect = struct_mod::Rectangle {
        width: 20,
        height: 30,
    };
    println!("area:{}", rect.area());
    dbg!(&rect);
    println!("rect1 is {:?}", rect);

    println!("area:{}", struct_mod::Rectangle::square(10).area());

    // enum_mod ====================================

    println!("{:?}", enum_mod::Ip::V4);
    println!("{:?}", enum_mod::Ip::V6);

    enum_mod::value_in_cents(enum_mod::Coin::Quarter(enum_mod::UsState::Alabama));

    enum_mod::equal();

    module_mod::eat_at_restaurant();
    back_of_house::Breakfast::summer("xxx");

    vec_mod::vec_logic();
    vec_mod::for_vec();
    vec_mod::difftype_vec();

    string_mod::str_const_to_string();
    string_mod::str_method();
    string_mod::str_add();
    string_mod::str_format();
    string_mod::str_index();
    string_mod::str_split();

    hashmap_mod::map_new();

    // error_mod::err_panic();

    // std_mod::fs::open::open();
    fs::open();
    // fs::fast_open();
    let _ = fs::open_res();
    let _ = fs::open_res_chain();
    // match fs::std_read_file(){
    //     Ok()
    // };
    // 
    
}
