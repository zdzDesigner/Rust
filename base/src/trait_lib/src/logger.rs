use std::fmt::Display;

trait Logger {
    fn log(&self, level: u8, message: impl Display);
}

struct Log;

impl Logger for Log {
    fn log(&self, level: u8, message: impl Display) {
        // println!("level:{}, message:{}", level, message);
        println!("level:{}, message:{message}", level);
    }
}

struct LogFilter {
    inner: Log,
    max_level: u8,
}
impl Logger for LogFilter {
    fn log(&self, level: u8, message: impl Display) {
        if level < self.max_level {
            self.inner.log(level, message);
        }
    }
}

fn trace(l: &impl Logger) {
    l.log(4, String::from("444vvv"));
    l.log(1, String::from("11111vvv"));
}

pub fn run_log() {
    let logfilter = LogFilter {
        max_level: 3,
        inner: Log,
    };
    trace(&logfilter);
}
