use std::fs::File;
use std::io::Read; // 手动引入，不然Lsp 无法提示

pub fn open() {
    let f = File::open("/home/zdz/Documents/Try/Rust/course/base/src/std_mod/fs/open.rs");

    println!("f:{:?}", f);

    let mut f = match f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => match File::create(
                "/home/zdz/Documents/Try/Rust/course/base/src/std_mod/fs/open.md",
            ) {
                Ok(file) => file,
                Err(err) => panic!("create file err:{:?}", err),
            },
            other => panic!("err:{:?}", other),
        },
    };
    println!("f:{:?}", f);
    let mut bufstr = String::new();

    match f.read_to_string(&mut bufstr) {
        Ok(_) => {
            println!("{}", bufstr)
        }
        Err(err) => panic!("read file:{:?}", err),
    };
}

pub fn open_or() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

pub fn fast_open() {
    // let f = File::open("hello.txt").unwrap(); // 快速处理错误
    // println!("f:{:?}", f);

    let mut f = File::open("hello.txt").expect("custom config error");
    println!("f:{:?}", f);

    let mut buf = String::new();
    let _ = f.read_to_string(&mut buf);
}

pub fn open_res() -> Result<String, std::io::Error> {
    let mut buf = String::new();
    let mut f = File::open("/home/zdz/Documents/Try/Rust/course/base/src/std_mod/fs/open.md")?;
    f.read_to_string(&mut buf)?;
    println!("buf:{}", buf);
    return Ok(buf);
}
pub fn open_res_chain() -> Result<String, std::io::Error> {
    let mut buf = String::new();
    File::open("/home/zdz/Documents/Try/Rust/course/base/src/std_mod/fs/open.md")?
        .read_to_string(&mut buf)?;
    println!("buf:{}", buf);
    return Ok(buf);
}

pub fn std_read_file() -> Result<String, std::io::Error> {
    std::fs::read_to_string("hello.txt")
}
