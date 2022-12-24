
extern crate regex;
mod url;

fn main() {
    let url = url::Url::parse("https://user@www.domain.com:3232/패스/사용자?이름=아무개&나이=32").expect("invalid url");

    println!("{:?}", url);
    
}
