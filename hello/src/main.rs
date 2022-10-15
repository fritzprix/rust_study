
use std::io::Write;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m !=0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn main() {
    let mut numbers: Vec<u64> = vec![];
    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str_radix(&arg, 10).expect("error parsing arguments"))
    }
    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd Number ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    println!("GCD of {:?} is {}", numbers, d);
}

#[test]
fn test()  {
    assert!(gcd(1, 2) == 1)
}
