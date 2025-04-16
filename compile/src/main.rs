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

mod generics;
mod lifetime;
mod move_demo;
mod mutil_mut;
mod stack;
mod closure;

fn main() {
    // stack::basetype();
    //
    // generics::pure(3);
    // generics::pure('a');
    // generics::pure("aaa");

    // mutil_mut::err_mut();
    // move_demo::test_move();
    // lifetime::longest("aaa", "bbb");
    closure::test_closure_scope();
}
