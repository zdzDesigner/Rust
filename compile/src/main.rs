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

fn main() {
    let x = 5;
    let y = 10;
    let z = if x > y { x + y } else { x * y };
    println!("Result: {}", z);
}
