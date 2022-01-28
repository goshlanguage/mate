#[cfg(test)]
mod tests {
    use crate::state::api::get_authority;

    #[test]
    fn test_get_authority() {
        std::env::set_var("AUTHORITY", "https://lol.com/");
        assert_eq!(get_authority(), "https://lol.com");
    }
}
