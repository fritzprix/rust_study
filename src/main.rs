extern crate collection;
use collection::map;

fn main() {
    let numbeers  = vec![1,2,3,4,5,6];
    let anoterh_number = vec![7,8,9,10,11,12];
    let mut number_tree = map::binary_tree::new();
    let mut another_number_tree = map::binary_tree::new();

    for i in &numbeers {
        number_tree.insert(*i);
    }

    number_tree.print();

    number_tree = number_tree.join(another_number_tree);
    number_tree.print();

}