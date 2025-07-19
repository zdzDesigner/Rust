use std::collections::HashMap;

pub fn map_new() {
    let mut map: HashMap<String, u16> = HashMap::new();

    map.insert(String::from("xxx"), 3);
    map.insert(String::from("yyy"), 32);
    map.insert(String::from("xxx"), 333); // 直接替换
    map.entry(String::from("xxx")).or_insert(3); // 先判断是否存在, 不存在插入
    let v = map.entry(String::from("zzz")).or_insert(3); // 先判断是否存在, 不存在插入
    *v += 10; // 返回这个键的值的一个可变引用（`&mut V`）

    println!("map:{:?}", map);
    println!("map:{}", map.len());

    match map.get(&String::from("xxx")) {
        Some(v) => {
            println!("Option::v:{}", v); // 3
        }
        None => {
            println!("nothing");
        }
    };

    for (key, val) in &map {
        println!("key:{},val:{}", key, val);
    }
}

#[cfg(test)]
mod test_hashmap_zip {
    use std::collections::HashMap;

    #[test]
    fn haspmap_zip() {
        let ks = [2, 83, 1, 9];
        let vs = ["aa", "dd", "cc", "bb"];

        let map: HashMap<&u32, &str> = ks.iter().zip(vs).collect();
        println!("map:{map:?}");
        // let vs2 = vs.iter().map(|v| v.to_string());
    }
}
