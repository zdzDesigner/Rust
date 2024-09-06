pub fn logic() {
    base();
}

fn base() {
    assert_eq!(format!("{}", 2u32 + 3), "5");
    assert_eq!(format!("{}", 2i32 - 3), "-1");
    assert_eq!(format!("{}", true && false), "false");
    assert_eq!(format!("{}", true || false), "true");
    assert_eq!(format!("{}", !true), "false");
    assert_eq!(format!("{:06b}", 0b0011u32 & 0b0101), "000001");
    assert_eq!(format!("{}", 1_000_000u32), "1000000");
}
