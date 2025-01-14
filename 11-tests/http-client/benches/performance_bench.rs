use criterion::{criterion_group, criterion_main, Criterion};
use http_client::HttpClient;

pub fn bench_get_request(c: &mut Criterion) {
    c.bench_function("GET Request", |b| {
        b.iter(|| {
            HttpClient::get("http://example.com").unwrap();
        })
    });
}

criterion_group!(benches, bench_get_request);
criterion_main!(benches);
