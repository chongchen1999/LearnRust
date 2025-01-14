use http_client::HttpClient;

#[test]
fn test_end_to_end_request() {
    let url = "http://example.com";
    let result = HttpClient::get(url);
    assert_eq!(result.unwrap(), "Hello, World!");
}
