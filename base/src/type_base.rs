#[cfg(test)]
mod test_type_base {
    #[test]
    fn test_tuple() {
        let color: (u8, u8, u8, f32) = (255, 44, 88, 0.1);
        println!("{:?}", color);

        let red = color.0;
        println!("{}", red);

        let (red, green, blue, alpha) = color;
        println!("{}", alpha);

        // for v in color { // not an iterator
        //     println!("{}", v);
        // }

        // for v in Some(color) {
        //     println!("v:{:?}", v); // v:(255, 44, 88, 0.1)
        // }
    }

    #[test]
    fn test_array() {
        let names = ["zdz", "zym"];
        println!("names len:{}", names.len());
    }
}
