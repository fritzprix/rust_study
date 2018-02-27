
mod tree;
use tree::Tree;


fn main() {
    let names = vec!["David","Romeo","John","Charlie"];
    let ages = vec![25,32,42,55];
    let mut name_tree: Tree<&str> = Tree::new();
    let mut age_tree: Tree<i32> = Tree::new();
    for name in &names {
        name_tree.insert(*name);
    }
    for age in &ages {
        age_tree.insert(*age);
    }

    name_tree.print();
    age_tree.print();
    match name_tree.has(names[1]) {
        true => println!("name_tree has {}",names[1]),
        false => println!("name_tree doesn't has {}",names[1])
    }

    match name_tree.remove_min() {
        None => println!("Empty"),
        Some(removed) => println!("{:?} is removed", removed)
    }
    name_tree.print();

    match name_tree.remove_max() {
        None => println!("Empty"),
        Some(removed) => println!("{:?} is removed", removed)
    }

    name_tree.print();


    match (names.len(),name_tree.size()) {
        (name_len, tree_len) if name_len == tree_len => {
            println!("{} = {} => Size Matched !!",name_len, tree_len);
        },
        (name_len, tree_len) if name_len != tree_len => {
            println!("{} != {} Size Not Matched !!",name_len, tree_len);
        },
        _ => {
            panic!("Something going wrong with tree!!");
        }
    }

}
