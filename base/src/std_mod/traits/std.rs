#[cfg(test)]
mod test_std_trait {
    struct Arg {
        isclip: bool,
    }
    impl Arg {
        fn new(isclip: bool) -> Self {
            Arg { isclip }
        }
    }

    #[test]
    fn eq_trait() {}
}
