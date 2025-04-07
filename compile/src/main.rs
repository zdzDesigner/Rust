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

fn main() {
    stack::basetype();

    generics::pure(3);
    generics::pure('a');
    generics::pure("aaa");

}
