fn main() {
    let a = String::from("aaaa");
    let b = &a;
    // print!("{}{}", b, a);

    mv();
}

fn mv() {
    let a = String::from("aaaa");
    let b = a;
    // print!("b:{}", b);
}
