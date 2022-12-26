use std::{num::ParseIntError, fmt::Debug};

fn main() {
    let result = panic_example();
    println!("Result: {}", result);
}

fn multiply_graceful(first:&str, second: &str) -> Result<i32, ParseIntError> {
    let first = first.parse::<i32>()?;
    let second = second.parse::<i32>()?;
    Ok(first * second)
}

fn divide_graceful(first:&str, second:&str) -> Result<i32, ParseIntError> {
    first.parse::<i32>().and_then(|first_no| {
        second.parse::<i32>().map(|second_no| first_no / second_no)
    })
}

fn multiply_ungraceful(first:&str, second:&str) -> Result<i32, ParseIntError> {
    let first_no = first.parse::<i32>().expect("should be integer string for first");
    let second_no = second.parse::<i32>().unwrap();
    Ok(first_no * second_no)
}

trait Printer {
    fn print(&self);
}

impl <R,E> Printer for Result<R,E> where R: Debug, E: Debug {
    fn print(&self) {
        match self {
            Ok(v) => println!("{:?}", v),
            Err(e) => println!("{:?}", e)
        }
    }
}


fn panic_example() -> i32 {

    let twenty = multiply("10", "2");
    twenty.print();

    let tt = multiply_var_2("t", "2");
    tt.print();

    let x = vec![1, 2, 3];
    let result = std::panic::catch_unwind(|| {
        x[99];  // this will cause a panic
        return 4;
    });
    match result {
        Ok(val) => val,
        Err(e) => {
            println!("Caught panic! {:?}", e);
            return 5;
        }
    }
}
