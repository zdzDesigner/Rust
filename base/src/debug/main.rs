pub fn main() {
    let name = "zdz";
    let ptr = name.as_ptr();
    println!("{ptr:?}");
    let first = name.chars().next().unwrap();
    println!("{first:?}");
}
