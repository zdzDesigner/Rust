#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused)]

use std::env::args;
use std::fs;
use std::{env, error::Error};

#[derive(Debug)]
pub struct Config {
    filename: String,
    query_text: String,
}
// pub struct Config<'a> {
//     pub filename: &'a str,
//     pub query_text: &'a str,
// }

impl Config {
    // pub fn new(args: &[String]) -> Result<Config, &'static str> {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // 'static 可以省略
        // let conf = Config {
        //     filename: &args[1],
        //     query_text: &args[2],
        // };

        // if let Some(filename) = args.get(1) {}
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
    let Some(filename) = args_iter.next() else {
        panic!("must has filename");
    };
    let Some(query_text) = args_iter.next() else {
        panic!("must has query text");
    };
    println!("filename:{:?}, query_text:{:?}", filename, query_text);
}

fn getArgsOfMatch() -> Result<String, &'static str> {
    let mut args_iter = env::args();
    args_iter.next();
    let filename = match args_iter.next() {
        Some(filename) => filename,
        None => return Err("xxx"),
    };

    let query_text = match args_iter.next() {
        Some(v) => v,
        None => return Err("xxx"),
    };

    Ok(String::from("ok"))
}

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

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
