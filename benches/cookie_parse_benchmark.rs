use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

use cookie_parse::cookie::*;

const COOKIE: &str = "Cookie: PHPSESSID=298zf09hf012fh2; csrftoken=u32t4o3tb3gg43; _gat=1";

fn bench_parse_cookies(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parse Cookies");
    group.bench_function("Cookie Parse 1st Attempt", |b| {
        b.iter(|| parse_cookies1(black_box(COOKIE)))
    });
    group.bench_function("Cookie Parse 2nd Attempt", |b| {
        b.iter(|| parse_cookies2(black_box(COOKIE)))
    });
    group.bench_function("Cookie Parse 3rd Attempt", |b| {
        b.iter(|| parse_cookies3(black_box(COOKIE)))
    });
    group.bench_function("Cookie Parse 4th Attempt", |b| {
        b.iter(|| parse_cookies4(black_box(COOKIE)))
    });
    group.bench_function("Cookie Parse Final", |b| {
        b.iter(|| parse_cookies(black_box(COOKIE)))
    });
    group.finish();
}

criterion_group!(benches, bench_parse_cookies);
criterion_main!(benches);
