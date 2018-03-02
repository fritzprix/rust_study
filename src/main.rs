extern crate collection;
extern crate rand;

use rand::Rng;
use collection::map::Map;
use collection::map::binary_tree;

fn main() {
    let f : Vec<i32>;
    let mut rng = rand::thread_rng();
    let mut numbers :Vec<i32>  = vec![];
    for i in 0..100 {
        numbers.push(rng.gen_range(0,10000));
    }
    let anoterh_number = vec![7,8,9,10,11,12];
    let mut number_tree  = binary_tree::new();
    let mut another_number_tree = binary_tree::new();

    for i in &numbers {
        number_tree.insert(*i);
    }

    number_tree.print();

    let fnc = |v : i32| println!("{:?}",v);
    for number in &numbers {
        println!(" ");
        number_tree.remove(*number).map(|v| println!("Removed {:?}", v));
        number_tree.print();
        println!(" ");
    }

    number_tree = number_tree.join(another_number_tree);
    number_tree.print();

}