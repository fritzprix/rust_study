use std::{io, cmp::Ordering};
use rand::{self, Rng};

fn main() {
    let mut line: String = String::new();
    let secret: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");
    let mut count: u8 = 254u8;
    loop {
        line.clear();
        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read input");
        match line.trim().parse() as Result<u32, _>{
            Ok(num) => {
                
                match secret.cmp(&num) {
                    Ordering::Equal => {
                        println!("You bet!: {num} = {secret}");
                        break;
                    },
                    Ordering::Greater => {
                        println!("more");
                    },
                    Ordering::Less => {
                        println!("less");
                    }
                }
            },
            Err(_) => {
                count = count.wrapping_add(1u8);
                println!("Try positive integer value (1 ~ 100) (incorrection count {count})");
            }
        }
    };
}
