use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

#[cfg(test)]
mod test_enum_result {
    #[test]
    fn test_result() -> Result<(), String> {
        return Err(String::from("this is error test!"));
    }

    #[test]
    fn unwrap_or_else() {
        Err(String::from("this is error test!")).unwrap_or_else(|err| {
            println!("unwrap or else:{:?}", err);
        })
    }

    fn result_unit_type() -> Result<(), String> {
        return Err(String::from("this is error test!"));
    }
    fn result(isok: bool) -> Result<String, String> {
        if isok {
            Ok("result ok".to_string())
        } else {
            Err("result error".to_string())
        }
    }

    #[test]
    fn test_let_result() -> () {
        if let Ok(v) = result(true) {
            println!("{:?}", v);
        }

        let Ok(v) = result(true) else { return };
        println!("{:?}", v);
    }
}
