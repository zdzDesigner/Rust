#![allow(unused)]

#[derive(Debug)]
pub enum Ip {
    V4,
    V6,
}

#[derive(Debug)]
pub enum UsState {
    Alabama,
    Alaska,
}

pub enum Coin {
    Dime,
    Quarter(UsState),
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
                UsState::Alabama => {
                    println!("{:?}", state);
                }
                UsState::Alaska => {
                    println!("{:?}", state);
                }
            }
            25
        }
    }
}

pub fn equal() {
    if let Some(3) = Some(3u8) {
        println!("three");
    }
    if let Some(3u8) = Some(3) {
        println!("three");
    }
}
