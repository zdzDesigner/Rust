// fn main() {
//     let a = String::from("aaaa");
//     let b = &a;
//     // print!("{}{}", b, a);
//
//     mv();
// }
//
// fn mv() {
//     let a = String::from("aaaa");
//     let b = a;
//     // print!("b:{}", b);
// }

mod stack;
mod generics;
mod mutil_mut;
mod move_demo;

fn main() {
    // stack::basetype();
    //
    // generics::pure(3);
    // generics::pure('a');
    // generics::pure("aaa");

    // mutil_mut::err_mut();
    move_demo::test_move();
}
