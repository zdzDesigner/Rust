use std::any::type_name;

pub fn type_of<T>(t: T) {
    println!("type_name:{}", type_name::<T>());
}

#[cfg(test)]
mod test_type_base {
    use super::*;

    #[test]
    fn test_tuple() {
        let color: (u8, u8, u8, f32) = (255, 44, 88, 0.1);
        println!("{:?}", color);

        let red = color.0;
        println!("{}", red);

        let (red, green, blue, alpha) = color;
        println!("{}", alpha);

        type_of(red); // u8
        // for v in color { // not an iterator
        //     println!("{}", v);
        // }

        // for v in Some(color) {
        //     println!("v:{:?}", v); // v:(255, 44, 88, 0.1)
        // }
    }
    #[test]
    fn test_str() {
        let name = "zdz";
        type_of(name); // &str
        println!("name ptr:{:p}", name.as_ptr()); // 0x5615509c9440
        println!("name len:{}", name.len());

        let second_ptr = unsafe { name.as_ptr().add(1) };
        println!("name second_ptr:{:p}", second_ptr); // 0x55be3c4b9441

        let name_bytes = name.as_bytes();
        type_of(name_bytes); // &[u8]
        println!("{name_bytes:?}"); // [122, 100, 122]
        // name[0]
        println!("{:?}", name.chars().next());
        println!("{}", name.chars().next().unwrap());
        println!("z");

        let arr = [10, 20, 30, 40]; // Array
        println!("arr ptr:{:?}", arr.as_ptr());
        println!("{:?}", arr.iter().next());
    }

    #[test]
    fn test_array() {
        let names = ["zdz", "zym"];
        println!("names len:{}", names.len());
        type_of(names); // [&str; 2]
    }
}
