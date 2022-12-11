mod collection;

use collection::{list::List, Queue};

fn main() {
    let mut customes = List::new();
    for i in 0..5 {
        customes.enqueue(i.to_string());
    }

    for c in customes.iter() {
        println!("{:?}", c);
    }

    while let Some(c) = customes.dequeue() {
        println!("{:?}", c);
    }

}

