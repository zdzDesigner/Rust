#[cfg(test)]
mod test_env_mod {
    #[test]
    fn test_env() {
        let Ok(user) = std::env::var("USER") else {
            return;
        };

        println!("env var USER:{:?}", user);
    }
}
