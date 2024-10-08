#[cfg(test)]
mod tests {
    use crate::utils::is_root;
    use std::env;

    #[test]
    fn test_is_root() {
        if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
            assert_eq!(is_root(), env::var("USER").unwrap() == "root");
        }
    }
}
