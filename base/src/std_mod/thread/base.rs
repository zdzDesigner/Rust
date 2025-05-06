#[cfg(test)]
mod test_thread {
    use std::time::Duration;

    #[test]
    fn base() {
        std::thread::spawn(|| {
            eprintln!("sub thread");
            for i in 0..10 {
                eprintln!("i:{:?}", i);
                std::thread::sleep(Duration::from_millis(100));
            }
        });

        println!("man thread!");
        std::thread::sleep(Duration::from_secs(100));
    }
}

// cargo t test_thread --  --nocapture --show-output

