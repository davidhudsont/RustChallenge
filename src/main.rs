mod cookie;

use cookie::*;

// https://confluence.metro.ad.selinc.com/pages/viewpage.action?pageId=1921190652
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cookie

// Example: 'Cookie: name=value; name2=value2'

pub fn main(){
    let c = parse_cookies("Cookie: name=value; name2=value2").unwrap();
    println!("{}", print_cookies(c));
}

