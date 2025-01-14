use http_client::HttpClient;

fn main() {
    let url = "http://example.com";
    match HttpClient::get(url) {
        Ok(response) => println!("Response: {}", response),
        Err(err) => eprintln!("Error: {}", err),
    }
}
