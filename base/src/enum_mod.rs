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
    fn test_write_message() {
        let msg = Message::Write(String::from("hello"));
        msg.call();
    }
}
