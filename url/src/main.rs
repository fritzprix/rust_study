
extern crate regex;
mod url;

fn main() {
    let url = url::Url::parse("https://user@www.domain.com:3232/path1/path2/path3?name=david&age=23#about").expect("invalid url");
    println!("{:?}", url);
    
}
