use cookie_parse::cookie::{parse_cookies, print_cookies};

// https://confluence.metro.ad.selinc.com/pages/viewpage.action?pageId=1921190652
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cookie

// Example: 'Cookie: name=value; name2=value2'

#[derive(Debug)]
struct Item;
type Id = String;

fn find(id: &Id) -> Result<Item, String> {
    Err(format!("Not found: {:?}", id))
}

fn find2(id: &u32) -> Option<u32> {
    if *id == 1 {
        Some(42)
    } else {
        None
    }
}

pub fn main() {
    let c = parse_cookies("Cookie: name=value; name2=value2").unwrap();
    println!("{}", print_cookies(c));

    let s = |s: &str| s.to_string();
    let ids = vec![s("1"), s("2"), s("3")];

    let items: Result<Vec<_>, _> = ids.iter().map(find).collect();
    println!("Result: {:?}", items);

    let ids = vec![1, 2, 3];
    let items: Option<Vec<_>> = ids.iter().map(find2).collect();
    println!("Result: {:?}", items);
}
