// mod print;
// mod tuple;
// mod typebase;
// mod ownership;
mod struct_mod;

fn main() {
    // print::print();
    // tuple::logic();
    // typebase::logic();
    // ownership::ownership();
    let rect = struct_mod::Rectangle {
        width: 20,
        height: 30,
    };
    println!("area:{}", rect.area());
    dbg!(&rect);
    println!("rect1 is {:?}", rect);

    println!("area:{}", struct_mod::Rectangle::square(10).area());
}
