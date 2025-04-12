#![allow(non_snake_case)]
#![allow(dead_code)]

use std::env;

#[derive(Debug)]
struct Config<'a> {
    filename: &'a str,
    query_text: &'a str,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args:{:?}", args);

    let filename = &args.get(1).expect("must has filename");
    let query_text = &args.get(2).expect("must has query text");
    let conf = Config {
        filename,
        query_text,
    };
    // let conf = Config {
    //     filename: &args[1],
    //     query_text: &args[2],
    // };
    println!("{:#?}", conf);
}

fn getArgs() {
    let mut args_iter = env::args();
    args_iter.next();
    let filename = args_iter.next();
    if filename == None {
        panic!("must has filename");
    }
    let query_text = args_iter.next();
    if query_text == None {
        panic!("must has query text");
    }
    println!("filename:{:?}, query_text:{:?}", filename, query_text);
}

fn getArgsOfVec() {}
