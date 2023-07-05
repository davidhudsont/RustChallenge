use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

#[inline]
pub fn parse_cookies1(cookie_line: &str) -> Option<HashMap<String, String>> {
    if !cookie_line.starts_with("Cookie:") {
        return None;
    }

    let cookies: Vec<_> = cookie_line
        .trim_start_matches("Cookie:")
        .split(";")
        .map(|s| s.trim())
        .collect();

    let mut hash_map = HashMap::new();

    for cookie in &cookies {
        let parsed_cookie: Vec<_> = cookie.split("=").collect();
        hash_map.insert(parsed_cookie[0].to_string(), parsed_cookie[1].to_string());
    }

    Some(hash_map)
}

#[inline]
pub fn parse_cookies2(cookie_line: &str) -> Option<HashMap<String, String>> {
    if !cookie_line.starts_with("Cookie:") {
        return None;
    }

    let cookies: Vec<_> = cookie_line
        .trim_start_matches("Cookie:")
        .split(";")
        .map(|s| s.trim())
        .collect();

    Some(
        cookies
            .iter()
            .map(|&cookie| {
                let parsed_cookie: Vec<_> = cookie.split("=").collect();
                (parsed_cookie[0].to_string(), parsed_cookie[1].to_string())
            })
            .collect(),
    )
}

#[inline]
pub fn parse_cookies3(cookie_line: &str) -> Option<HashMap<String, String>> {
    if !cookie_line.starts_with("Cookie:") {
        return None;
    }

    Some(
        cookie_line
            .trim_start_matches("Cookie:")
            .split(";")
            .map(|s| s.trim())
            .collect::<Vec<_>>()
            .iter()
            .map(|&cookie| {
                let parsed_cookie: Vec<_> = cookie.split("=").collect();
                (parsed_cookie[0].to_string(), parsed_cookie[1].to_string())
            })
            .collect(),
    )
}

#[inline]
pub fn parse_cookies4(cookie_line: &str) -> Option<HashMap<String, String>> {
    if !cookie_line.starts_with("Cookie: ") {
        return None;
    }

    let pairs: Vec<_> = cookie_line
        .trim_start_matches("Cookie: ")
        .split("; ")
        .collect(); // ["name=value", "name2=value2"]

    let split_pairs: Vec<(&str, &str)> = pairs
        .iter()
        .filter_map(|&cookie| cookie.split_once("="))
        .collect();

    Some(
        split_pairs
            .iter()
            .map(|pair| (pair.0.to_owned(), pair.1.to_owned()))
            .collect(),
    )
}

#[inline]
pub fn parse_cookies(cookie_line: &str) -> Option<HashMap<String, String>> {
    cookie_line.strip_prefix("Cookie: ").map(|cookie| {
        cookie
            .split("; ")
            .filter_map(|c| c.split_once('='))
            .map(|p| (p.0.to_owned(), p.1.to_owned()))
            .collect()
    })
}

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
