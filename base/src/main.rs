// mod print;
// mod tuple;
// mod typebase;
// mod ownership;
mod enum_mod;
mod struct_mod;
mod module_mod;

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
}
