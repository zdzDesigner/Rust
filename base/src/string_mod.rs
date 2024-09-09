pub fn str_const_to_string() {
    let s_obj = "aaaa str_const_to_string".to_string();
    println!("{}", s_obj); // aaaa str_const_to_string
    println!("{}", s_obj.to_string()); // aaaa str_const_to_string
    println!("{}", s_obj.as_str()); // aaaa str_const_to_string
}

pub fn str_method() {
    let mut s = String::from("x");

    s.push_str("sss");

    println!("s:{}", s); // sss
    println!("s:{:?}", s); // "sss"
    println!("s:{:?}", s.as_bytes());

    println!("&s[0..]:{}", &s[0..3]);
}

pub fn str_add() {
    let s1 = String::from("aaaa");
    let s2 = String::from("bbbb");

    let s3 = s1 + &s2;

    println!("s3:{}", s3);
    // println!("s1:{}", s1); // s1 所有权丢失
}
pub fn str_format() {
    let s1 = String::from("aaaa");
    let s2 = String::from("bbbb");

    println!("format!:{}", format!("{}{}", s1, s2));
    println!("s1:{}", s1);
    println!("s2:{}", s2);
}

pub fn str_index() {
    let normal = "3abcde";
    println!("normal s:{}", &normal[0..1]); // 3

    // ========================
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    println!("s:{}", s); // Зд
    println!("s:{}", &hello[0..2]); // 3

    for c in hello.bytes() {
        println!("for:{}", c);
    }
}

pub fn str_split() {
    let text = "hello world wonderful world";
    println!("{:?}", text.split_whitespace());
    println!("{:?}", text.split(' '));
}
