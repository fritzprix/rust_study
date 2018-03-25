extern crate collection;
use collection::map::{tree,Map};

fn main() {
    let mut my_tree = tree::BinaryTree::new();
    my_tree.insert(5);
    my_tree.insert(6);
    my_tree.insert(24);
    my_tree.insert(7);
    my_tree.insert(3);
    my_tree.insert(1);
    my_tree.insert(2);

    my_tree.print();

    println!("removed max : {:?}",my_tree.remove_max());
    my_tree.print();


    println!("removed min : {:?}",my_tree.remove_min());
    my_tree.print();

    if my_tree.remove(5) {
        println!("remove successful");
    } else {
        println!("remove fail");
    }

    my_tree.print();
}