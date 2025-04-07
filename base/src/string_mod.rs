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
    println!("s:{:?}", s.as_bytes()); // s:[120, 115, 115, 115]

    println!("&s[0..]:{}", &s[0..3]);
    // s.as_bytes().iter().next()
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
    println!("expect:{}", text.split(' ').next().expect("--")); // 如何为None 默认取"--"
    println!("unwrap:{}", text.split(' ').next().unwrap());
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // 空格
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// 返回String
pub fn ret_string() -> String {
    return format!("{}", "zdz");
}
pub fn ret_str(string: &str) -> &str {
    return string;
}
pub fn ret_static_str() -> &'static str {
    return "zdz";
}

#[cfg(test)]
mod test_string {
    use super::*;

    #[test]
    fn base() {
        str_method();
    }

    #[test]
    fn test_ret_string() {
        let string = ret_string();
        println!("string:{}", string);

        println!("str:{}", ret_str(&String::from("zdz")[0..]));
        println!("static_str:{}", ret_static_str());

        println!("first_word:{}", first_word(&String::from("hello world")));
    }

    #[test]
    fn parse() {
        let age = String::from("38 ");
        if let Ok(val) = age.parse::<usize>() {
            // 类型注释
            println!("{}", val);
        } else {
            println!("error!!!!");
        };
        println!("age.trim():{}", age.trim());
        println!("{:?}", age.trim().parse::<usize>()); // OK(38)
        println!("{:?}", age.trim().parse::<usize>().unwrap()); // 38
        println!("{:?}", age.trim().parse::<usize>().ok()); // Some(38)
        println!("{:?}", age.trim().parse::<usize>().ok().unwrap()); // 38
    }
}
