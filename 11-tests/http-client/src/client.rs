use crate::parser::parse_response;
use crate::validator::validate_url;

pub struct HttpClient;

impl HttpClient {
    pub fn get(url: &str) -> Result<String, String> {
        if validate_url(url) {
            let response = format!("{{\"status\":200, \"body\":\"Hello, World!\"}}");
            Ok(parse_response(&response).unwrap_or_else(|_| "Error parsing response".to_string()))
        } else {
            Err("Invalid URL".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_get_request() {
        let url = "http://example.com";
        let result = HttpClient::get(url);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, World!");
    }

    #[test]
    fn test_invalid_get_request() {
        let url = "invalid-url";
        let result = HttpClient::get(url);
        assert!(result.is_err());
    }
}
