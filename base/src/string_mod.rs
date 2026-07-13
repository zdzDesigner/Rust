use crate::type_base::type_of;

pub fn str_method() {
    let mut s = String::from("x");

    type_of(s.clone());
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
        let number = 6;
        let result = if number % 2 == 0 { "偶数" } else { "奇数" };

        str_method();
    }

    #[test]
    fn test_ret_string() {
        let string = ret_string();
        println!("string:{}", string);

        println!("str:{}", ret_str(&String::from("zdz")[0..]));
        println!("str:{}", ret_str(&String::from("zdz")));
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

    #[test]
    fn split() {
        let ips = String::from("127.0.0.1/127.0.0.2/127.0.0.3/127.0.0.4");
        let res = ips.split("/").next().unwrap();
        println!("res:{:?}", res);

        // vec
        let list: Vec<&str> = ips.split("/").collect();
        println!("list:{:?}", list);
        type_of(&list);
        for v in &list {
            println!("{v}");
        }
        let mut iter2 = list.iter();
        while let Some(v) = iter2.next() {
            println!("while: {v}");
        }

        // iter
        let mut iter = ips.split("/").into_iter();
        type_of(iter.clone());
        println!("nth:{:?}", iter.nth(2)); // 跳转到第二个元素(前面的元素已被消费掉)
        let res = iter.next();
        println!("res:{:?}", res);
        let res = iter.next();
        println!("res:{:?}", res);
        let res = iter.next();
        println!("res:{:?}", res);
        let res = iter.next();
        println!("res:{:?}", res);
        let res = iter.next().unwrap_or("默认值");
        println!("res:{:?}", res);
        type_of(iter.clone());
        // let res = iter.next().expect("panic了, 这条信息会打印");
        // println!("res:{:?}", res);
    }
    #[test]
    fn str_to_string() {
        let mut s_obj = "aaaa".to_string();
        println!("{}", s_obj); // aaaa
        println!("{}", s_obj.to_string()); // aaaa
        println!("{}", s_obj.as_str()); // aaaa
        println!("{:?}", s_obj.as_bytes()); // [97, 97, 97, 97]
        s_obj.push('!');
        println!("{}", s_obj); // aaaa
    }
    #[test]
    fn change_string() {
        let s_obj = String::from("aaaa");
        // let ptr = unsafe { s_obj.as_ptr().add(1) };
    }
}

#[cfg(test)]
mod test_string_base {
    use crate::type_base::{print_type_of};


    #[test]
    fn test_string_base() {
        let name = "zdz";
        print_type_of(name);
        let name_obj = name.to_string();
        println!("name: {:?}", name);
        println!("name_obj: {:?}", name_obj);
        let vecname = name_obj.into_bytes();
        // println!("name_obj: {:?}", name_obj); // =========== 转移了

        println!("into_bytes:{:?}", vecname);
        // println!("into_bytes:{:?}", String::from_iter(vecname.iter()));
        println!("{:?}", String::from_utf8(vecname));
    }

    #[test]
    fn into_moved() {
        let name_obj = String::from("zdz");
        let name_vec = name_obj.into_bytes();
        // println!("moved: {:?}", name_obj); // =========== 转移了
    }

    #[test]
    fn string_unicode() {
        let zh = '中';
        println!("{zh} as u32:{}", zh as u32); // 20013
        println!("{zh} as u64:{}", zh as u64); // 20013
    }
}

#[cfg(test)]
mod test_multi_lines {
    #[test]
    fn test_lines() {
        let multi_lines = "\
common name,length (cm)
Little penguin,33
Yellow-eyed penguin,65
Fiordland penguin,60
Invalid,data";
        println!("{multi_lines}");
        for line in multi_lines.lines().enumerate() {
            println!("{line:?}");
        }
        for (i, text) in multi_lines.lines().enumerate() {
            println!("{i},{text}");
            println!("{:?}", text.split(','));
            for val in text.split(',') {
                println!("{val}");
            }
            // if let v = text.split(',').remainder() {
            //     println!("{:?}", v);
            // }
            // let list = text.split(',').map(|field| field.trim()).collect::<Vec<_>>();
            let list = text.split(',').collect::<Vec<_>>();
            println!("{list:?}");

            let count = list[0].parse::<usize>();
            println!("{count:?}");

            if let Ok(count) = list[1].parse::<i32>() {
                println!("== parse ok ==:{count}");
            }
        }
    }
}
