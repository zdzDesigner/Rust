#[cfg(test)]
mod mod_option {
    #[test]
    fn flatten() {
        {
            let x: Option<Option<Option<u32>>> = None;
            assert_eq!(x.flatten(), None);
        }

        {
            let x: Option<Option<Option<u32>>> = Some(Some(Some(6)));
            assert_eq!(x.flatten(), Some(Some(6)));
        }
        {
            let x: Option<Option<Option<u32>>> = Some(Some(Some(6)));
            assert_eq!(x.flatten().flatten(), Some(6));
        }
    }
}
