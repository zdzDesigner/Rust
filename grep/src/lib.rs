#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused)]

use std::fs;
use std::{env, error::Error};

#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub query_text: String,
}
// pub struct Config<'a> {
//     pub filename: &'a str,
//     pub query_text: &'a str,
// }

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // let conf = Config {
        //     filename: &args[1],
        //     query_text: &args[2],
        // };

        let Some(filename) = args.get(1) else {
            return Err("must has filename");
        };
        let Some(query_text) = args.get(2) else {
            return Err("must has query text");
        };
        println!("filename:{:?}, query_text:{:?}", filename, query_text);
        // let filename = args.get(1);
        // if None == filename {
        //     return Err("must has filename");
        // }
        // let query_text = args.get(2);
        // if None == query_text {
        //     return Err("must has query text");
        // }
        return Ok(Config {
            filename: filename.clone(),
            query_text: query_text.clone(),
        });
    }
}

pub fn getArgs() {
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

pub fn run(conf: &Config) -> Result<(), &str> {
    let content = fs::read_to_string(&conf.filename);
    println!("{:?}", content);

    if let Err(msg) = content {
        return Err("not found file!");
    }

    for line in content.unwrap().lines() {
        if line.contains("所示的") {
            println!("line:{:?}", line);
        }
    }

    Ok(())
}
