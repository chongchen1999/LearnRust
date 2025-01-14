use regex::Regex;

pub fn validate_url(url: &str) -> bool {
    let re = Regex::new(r"^https?://[a-zA-Z0-9\-.]+(:[0-9]+)?(/.*)?$").unwrap();
    re.is_match(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_url() {
        assert!(validate_url("http://example.com"));
    }

    #[test]
    fn test_invalid_url() {
        assert!(!validate_url("invalid-url"));
    }
}
