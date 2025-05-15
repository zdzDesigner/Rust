#![allow(unused)]

#[derive(Debug)]
pub enum Ip {
    V4,
    V6,
}

#[derive(Debug)]
pub enum ZhState {
    Beijing,
    Shanghai,
}

pub enum Coin {
    Dime,
    Quarter(ZhState),
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// 二次匹配
pub fn value_in_cents_two(coin: Coin) -> u8 {
    match coin {
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            match state {
                ZhState::Beijing => {
                    println!("{:?}", state);
                }
                ZhState::Shanghai => {
                    println!("{:?}", state);
                }
            }
            25
        }
    }
}

// Some
pub fn equal() {
    if let Some(3) = Some(3u8) {
        println!("three");
    }
    if let Some(3u8) = Some(3) {
        println!("three");
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Write(text) => {
                println!("{}", text);
            }
            _ => {}
        }
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum() {
        println!("Message::Quit = {:?}", Message::Quit);
        println!("Message::Quit = {:?}", Message::Quit);
    }

    #[test]
    fn test_write_message() {
        let msg = Message::Write(String::from("hello"));
        msg.call();
    }
}

#[cfg(test)]
mod test_enum_option {

    fn getname(isok: bool) -> Option<String> {
        // return None;
        // return Some(String::from("zdz"));
        if isok {
            return Some(String::from("zdz"));
        } else {
            return None;
        }
    }

    #[test]
    fn test_option() {
        if let Some(name) = getname(true) {
            println!("name:{:?}", name);
        }
        if let None = getname(false) {
            println!("isnone");
        }

        let name = getname(true);

        println!("{}", name.unwrap());
        // println!("{}", getname(false).unwrap()); // unwrap_failed() panic

        println!("{:?}", getname(false).unwrap_or("lmy".to_string()));
    }
}

#[cfg(test)]
mod test_enum_result {
    use std::error;

    fn isDynOK() -> Result<(), Box<dyn error::Error>> {
        return Ok(());
        // return Err("xxxxx"); // ??如何构造 error::Error
    }
    struct Error {}
    fn isOK() -> Result<(), Error> {
        // return Ok(());
        return Err(Error {});
    }
    #[test]
    fn test_result() {
        println!("result:{}", isOK().is_err());
    }
}
