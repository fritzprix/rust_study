extern crate collection;
extern crate rand;

use rand::Rng;
use collection::map::Map;
use collection::map::tree;

fn main() {
    let mut rng : rand::ThreadRng = rand::thread_rng();
    let mut numbers :Vec<i32>  = rng.gen_iter().map(|val| if val < 0 { val * -1 } else { val }).take(25).collect::<Vec<i32>>();
    let mut anoterh_numbers : Vec<i32> = rng.gen_iter().map(|val| if val < 0 { val * -1} else { val }).take(25).collect::<Vec<i32>>();
    let number_tree : tree::BinaryTree<i32>  = numbers.clone().into_iter().collect::<tree::BinaryTree<i32>>();
    let another_number_tree: tree::BinaryTree<i32> = anoterh_numbers.clone().into_iter().collect::<tree::BinaryTree<i32>>();


    for i in &numbers {
        assert!(number_tree.has(*i));
    }

    assert_eq!(number_tree.size(), numbers.len());

    // join two tree into one
    let mut joined_tree = number_tree.join(another_number_tree);

    // size should be sum of two
    assert_eq!(joined_tree.size(), numbers.len() + anoterh_numbers.len());

    numbers.append(&mut anoterh_numbers);

    for i in &numbers {
        match joined_tree.remove(*i) {
            Some(_) => {},
            None => {
                panic!("Unexpected None!!");
            }
        }
    }
}