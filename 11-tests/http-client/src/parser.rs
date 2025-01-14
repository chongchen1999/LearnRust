use serde_json::Value;
use serde::de::Error;

pub fn parse_response(response: &str) -> Result<String, serde_json::Error> {
    let v: Value = serde_json::from_str(response)?;
    match v["body"].as_str() {
        Some(body) => Ok(body.to_string()),
        None => Err(serde_json::Error::custom("Missing 'body' field")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_response() {
        let response = "{\"status\":200, \"body\":\"Success\"}";
        assert_eq!(parse_response(response).unwrap(), "Success");
    }

    #[test]
    fn test_parse_invalid_response() {
        let response = "{\"status\":200, \"data\":\"Missing body\"}";
        assert!(parse_response(response).is_err());
    }
}
