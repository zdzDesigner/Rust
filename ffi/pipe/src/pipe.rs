use serde::Deserialize;
// use serde_json::Value;
use std::collections::HashMap;
use std::io::{self, Write};

type Handler = fn(&str);
pub trait Event {
    fn on(&mut self, evtname: String, handle: Handler);
    fn emit(&self, evtname: String, data: &str);
}

#[derive(Clone)]
pub struct Router {
    routes: HashMap<String, Handler>,
}
impl Router {
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }
}
impl Event for Router {
    fn on(&mut self, evtname: String, handle: Handler) {
        self.routes.insert(evtname, handle);
    }
    // pub fn emit(&self, evtname: String) -> Option<Handler> {}
    fn emit(&self, evtname: String, data: &str) {
        // eprintln!("evtname:{:?}", evtname);
        if let Some(handle) = self.routes.get(&evtname).copied() {
            handle(data);
        } else {
            eprintln!("hash map notfound!");
        }
        // match self.routes.get(&evtname) {
        //     Some(h) => *h,
        //     None => unreachable!(),
        // }
    }
}

#[derive(Debug, Deserialize)]
struct Payload {
    name: String,
    data: String,
}

pub fn listen(router: &Router) {
    let stdin = io::stdin();
    loop {
        let mut buf = String::new();
        eprintln!("read ...");
        if let Ok(size) = stdin.read_line(&mut buf) {
            eprintln!("size:{:?}", size);
            // eprintln!("buf:{}", buf);
            if let Ok(val) = serde_json::from_str::<Payload>(&buf) {
                // eprintln!("{:?}", val.name);
                // eprintln!("{:?}", val.data);
                router.emit(val.name, val.data.as_str());
            }
            // if let Ok(val) = serde_json::from_str::<Value>(&buf) {
            //     eprintln!("{}", val["name"]);
            //     eprintln!("iseql:{}", val["name"] == "probe");
            //     eprintln!("iseql:{}", val["name"] == "peq");
            //     router.emit(val["name"].to_string(), val["data"].as_str().unwrap())
            // }
            // if let Some(handle) = router.emit(val["name"].to_string()) {
            //     eprintln!("get handle ok!");
            //     handle(val["data"].as_str().unwrap());
            // }

            // 配合serde框架
            // if let Ok(val) = serde_json::from_str::<Payload>(&buf) {
            //     eprintln!("{:?}",val);
            // }

            // router.clone().emit(String::from("data"))(&buf);
            // router.emit(String::from("data"))(&buf);
        }

        // match io::stdin().read_line(&mut buf) {
        //     Ok(n) => {
        //         eprintln!("{n} bytes read");
        //         eprintln!("{buf}");
        //     }
        //     Err(error) => eprintln!("error: {error}"),
        // }
    }
}

pub fn send(data: &str) {
    let mut stdout = io::stdout();
    let _ = stdout.write_all(data.as_bytes());
}
