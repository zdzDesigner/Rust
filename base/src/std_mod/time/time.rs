#[cfg(test)]
mod test_time {
    use std::thread;
    use std::time;
    #[test]
    fn sleep() {
        println!("pre sleep!");
        thread::sleep(time::Duration::from_secs(3));
        println!("after sleep!");
    }
}
