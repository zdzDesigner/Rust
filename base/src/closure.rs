#[cfg(test)]
mod test_closure {
    #[test]
    fn test_closure_base() {
        let tofn = |num: u8| {
            println!("inner closure: {:?}", num);
        };
        tofn(8);
    }
}
