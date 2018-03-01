extern crate collection;
use collection::map;

fn main() {
    let numbers  = vec![1,5,4,7,52,3,74,142];
    let anoterh_number = vec![7,8,9,10,11,12];
    let mut number_tree = map::binary_tree::new();
    let mut another_number_tree = map::binary_tree::new();

    for i in &numbers {
        number_tree.insert(*i);
    }

    number_tree.print();

    if let true = number_tree.remove(numbers[2]) {
        println!("successfully removed !!");
    }

    number_tree = number_tree.join(another_number_tree);
    number_tree.print();

}