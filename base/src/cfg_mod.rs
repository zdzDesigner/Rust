#[cfg(feature = "xxx")]
pub fn cfg_xxx() {
    println!("xxx");
}

#[cfg(not(feature = "xxx"))]
pub fn cfg_xxx() {
    println!("xxx2");
}

