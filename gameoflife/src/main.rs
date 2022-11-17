
extern crate csv;
extern crate clap;
extern crate rand;

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    input: String
}

fn fill_random(map: &mut [u8]) -> usize {

    0
}

fn main() {
    let height = 40usize;
    let width = 40usize;
    let mut map = vec![0u8; height * width];
    
    println!("Hello, world!");
}
