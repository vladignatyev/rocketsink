use criterion::{criterion_group, criterion_main, Criterion};

extern crate eventsink;

use eventsink::sink::data::headermap_to_serde;
use rocket::http::HeaderMap;


fn criterion_benchmark(c: &mut Criterion) {
    let mut header = HeaderMap::new();
    header.add_raw("user-agent","curl/7.84.0");
    header.add_raw("x-test","Hello");
    header.add_raw("host","127.0.0.1");
    header.add_raw("accept","*/*");
    header.add_raw("content-type","application/json");

    c.bench_function("json_benchmark", |b| b.iter(||
        headermap_to_serde(&header).unwrap()
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
