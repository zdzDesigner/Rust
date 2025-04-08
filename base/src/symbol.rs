#[cfg(test)]
mod test_symbol {

    #[test]
    fn test_b() {
        let v = b"zdz";
        println!("{:?}", v);

        let v = b' ';
        if v == b' ' {
            println!("{:?}", v);
        }
    }

    fn first_word(word: &str) -> usize {
        // for item in word.to_string().into_bytes().iter() {
        for (i, &item) in word.as_bytes().iter().enumerate() {
            println!("{}", item);
            if item == b' ' {
                return i;
            }
        }
        return word.len();
    }

    fn first_word2(word: &str) -> &str {
        for (i, &item) in word.as_bytes().iter().enumerate() {
            if item == b' ' {
                return &word[..i];
            }
        }
        return &word[..];
    }
    #[test]
    fn test_first_word() {
        let mut statement = String::from("name vv d");
        let index = first_word(&statement);
        statement.clear();
        println!("index:{}", index);

        // error (memory intersection)
        // let mut statement2 = String::from("vv xx gg");
        // let substr = first_word2(&statement2);
        // statement2.clear(); // 已被清除
        //
        // println!("slice:{}", substr);
    }
}
