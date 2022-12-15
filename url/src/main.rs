
extern crate regex;
mod url;

fn main() {
    let url = url::URL::parse("http://www.naver.com:8080/user?name=david&age=24#about").expect("invalid url");
    println!("{:?}", url);
    
}
