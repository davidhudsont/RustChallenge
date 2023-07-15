// Only the Rust standard library can be used (std::)
use std::collections::HashMap;
use std::option::Option;
use std::string::String;

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
pub fn parse_cookies5(cookie_line: &str) -> Option<HashMap<String, String>> {
    cookie_line.strip_prefix("Cookie: ").map(|cookies| {
        cookies
            .split("; ")
            .filter_map(|cookie| cookie.split_once('='))
            .map(|cookie_pair| (cookie_pair.0.to_owned(), cookie_pair.1.to_owned()))
            .collect()
    })
}

fn parse_cookie_pair(cookie_pair: &str) -> Option<(String, String)> {
    cookie_pair
        .split_once("=")
        .and_then(|c| Some((c.0.to_owned(), c.1.to_owned())))
}

#[inline]
pub fn parse_cookies(cookie_line: &str) -> Option<HashMap<String, String>> {
    let semicolon_count = cookie_line.matches("; ").count();
    let equals_count = cookie_line.matches('=').count();
    if (equals_count == 0)
        || (semicolon_count == 0 && equals_count > 1)
        || (semicolon_count != (equals_count - 1))
    {
        return None;
    }

    cookie_line
        .strip_prefix("Cookie:")
        .and_then(|cookies| cookies.trim().split("; ").map(parse_cookie_pair).collect())
}

pub fn print_cookies(cookies: HashMap<String, String>) -> String {
    cookies
        .iter()
        .map(|cookie_pair| format!("{}: {}\n", cookie_pair.0, cookie_pair.1))
        .collect()
}

#[test]
fn test_given_example() {
    let c = parse_cookies("Cookie: name=value; name2=value2");
    assert_eq!(
        c,
        Some(HashMap::from([
            ("name2".to_string(), "value2".to_string()),
            ("name".to_string(), "value".to_string()),
        ]))
    );
}

#[test]
fn test_given_example_ows() {
    let c = parse_cookies("Cookie:name=value; name2=value2 ");
    assert_eq!(
        c,
        Some(HashMap::from([
            ("name2".to_string(), "value2".to_string()),
            ("name".to_string(), "value".to_string()),
        ]))
    );
}

#[test]
fn test_mozilla_example() {
    let c = parse_cookies("Cookie: PHPSESSID=298zf09hf012fh2; csrftoken=u32t4o3tb3gg43; _gat=1");
    assert_eq!(
        c,
        Some(HashMap::from([
            ("PHPSESSID".to_string(), "298zf09hf012fh2".to_string()),
            ("csrftoken".to_string(), "u32t4o3tb3gg43".to_string()),
            ("_gat".to_string(), "1".to_string()),
        ]))
    );
}

#[test]
fn test_single_cookie() {
    let c = parse_cookies("Cookie: name=value");
    assert_eq!(
        c,
        Some(HashMap::from([("name".to_string(), "value".to_string()),]))
    );
}

#[test]
fn test_many_cookies() {
    let c =
        parse_cookies("Cookie: name=value; name2=value2; name3=value3; name4=value4; name5=value5");
    assert_eq!(
        c,
        Some(HashMap::from([
            ("name5".to_string(), "value5".to_string()),
            ("name4".to_string(), "value4".to_string()),
            ("name3".to_string(), "value3".to_string()),
            ("name2".to_string(), "value2".to_string()),
            ("name".to_string(), "value".to_string()),
        ]))
    );
}

#[test]
fn test_reject_invalid_header_name() {
    let c = parse_cookies("Cooookie: name=value; name2=value2");
    assert_eq!(c, None);
}

#[test]
fn test_invalid_separator() {
    let c = parse_cookies("Cookie: name=value; name2]value2");
    assert_eq!(c, None);
}

#[test]
fn test_invalid_separator2() {
    let c = parse_cookies("Cookie: name]value");
    assert_eq!(c, None);
}

#[test]
fn test_invalid_separator3() {
    let c = parse_cookies("Cookie: name=value[ name2=value2");
    assert_eq!(c, None);
}

#[test]
fn test_invalid_separator4() {
    let c = parse_cookies("Cookie: name=value; name2=value2. name3=value3");
    assert_eq!(c, None);
}

#[test]
fn test_invalid_separator5() {
    let c = parse_cookies("Cookie: name=value;name2=value2");
    assert_eq!(c, None);
}
