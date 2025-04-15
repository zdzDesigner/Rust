#![allow(non_snake_case)]
#![allow(dead_code)]

use grep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args:{:?}", args);

    // let conf = Config::new(&args);
    // if let Err(errmsg) = conf {
    //     eprintln!("{:?}", errmsg);
    //     process::exit(1);
    // }
    // println!("{:#?}", conf);
    //
    // if let Err(errmsg) = grep::run(&conf.unwrap()) {
    //     eprintln!("{:?}", errmsg);
    //     process::exit(1);
    // }
    let Ok(conf) = Config::new(&args) else {
        // eprintln!("{:?}", errmsg);
        process::exit(1);
    };
    println!("conf:{:?}", conf);

    loop {}
}
