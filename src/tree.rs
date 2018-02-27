use std;


pub struct Tree<T:std::cmp::PartialOrd + std::fmt::Debug + std::clone::Clone> {
    entry : Option<Box<Node<T>>>
}

impl <T:std::cmp::PartialOrd + std::fmt::Debug + std::clone::Clone> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree {entry : None}
    }

    pub fn insert(&mut self, val : T) -> bool {
        let entry_node = &mut self.entry;
        match entry_node {
            &mut None => {
                *entry_node = Some(Box::new(Node::new(val)));
                true
            },
            &mut Some(ref mut entry_node) => entry_node.insert(val)
        }
    }

    pub fn print(&self) {
        match &self.entry {
            &None => println!("Emtry"),
            &Some(ref entry) => entry.print()
        }
    }

    pub fn has(&self, val : T) -> bool {
        match &self.entry {
            &None => false,
            &Some(ref entry_node) => entry_node.has(val)
        }
    }


    pub fn remove_max(&mut self) -> Option<T> {
        match &mut self.entry {
            &mut None => None,
            &mut Some(ref mut entry_node) => entry_node.remove_max()
        }
    }

    pub fn remove_min(&mut self) -> Option<T> {
        match &mut self.entry {
            &mut None => None,
            &mut Some(ref mut entry_node) => entry_node.remove_min()
        }
    }

    pub fn size(&self) -> usize {
        match &self.entry {
            &None => 0,
            &Some(ref entry_node) => entry_node.size()
        }
    }
}
#[derive(Clone)]
struct Node<T:std::cmp::PartialOrd + std::fmt::Debug + std::clone::Clone> {
    val: T,
    left : Option<Box<Node<T>>>,
    right : Option<Box<Node<T>>>
}


impl <T:std::cmp::PartialOrd + std::fmt::Debug + std::clone::Clone> Node<T> {
    fn new(init_val :T) -> Node<T> {
        Node {val : init_val, left: None, right : None}
    }

    fn remove_min (&mut self) -> Option<T> {
        let mut result : Option<T> = None;
        let mut del_subtree : bool = false;
        let mut replacement : Option<Box<Node<T>>> = None;
        match &mut self.left {
            &mut None => {},
            &mut Some(ref mut left) => {
                match left.remove_min() {
                    None => {
                        let cloned_left = left.clone();
                        del_subtree = true;
                        replacement = left.right.clone();
                        result = Some(cloned_left.val);
                    },
                    Some(ref removed_val) => {
                        result = Some(removed_val.clone());
                    }
                };
            }
        };
        if del_subtree {
            self.left = replacement;
        }
        result
    }

    fn remove_max (&mut self) -> Option<T> {
        let mut result : Option<T> = None;
        let mut del_subtree : bool = false;
        let mut replacement : Option<Box<Node<T>>> = None;
        match &mut self.right {
            &mut None => {},
            &mut Some(ref mut right) => {
                match right.remove_max() {
                    None => {
                        let mut cloned_right = right.clone();
                        del_subtree = true;
                        replacement = cloned_right.left.clone();
                        result = Some(cloned_right.val);
                    },
                    Some(ref removed_val) => {
                        result = Some(removed_val.clone());
                    }
                };
            }
        };
        if del_subtree {
            self.right = replacement;
        }
        result
    }

    fn insert(&mut self, val : T) -> bool {
        let target = if self.val > val {&mut self.left} else {&mut self.right};
        match target {
            &mut None => {
                *target = Some(Box::new(Node {val, left: None, right: None}));
                true
            },
            &mut Some(ref mut subtree) => subtree.insert(val)
        }
    }

    fn size(&self) -> usize {
        let mut size = 0;
        match &self.right {
            &None => {},
            &Some(ref right) => {
                size += right.size();
            }
        }
        match &self.left {
            &None => {},
            &Some(ref left) => {
                size += left.size();
            }
        }
        size + 1
    }

    fn has(&self, new_val : T) -> bool {
        match self {
            &Node {val : ref cur_val, left : _, ref right} if *cur_val < new_val => {
                match right {
                    &None => false,
                    &Some(ref right_child) => right_child.has(new_val)
                }
            },
            &Node {val : ref cur_val, ref left, right : _} if *cur_val > new_val => {
                match left {
                    &None => false,
                    &Some(ref left_child) => left_child.has(new_val)
                }
            },
            &Node {val : ref cur_val, left : _, right : _} if *cur_val == new_val => true,
            _ => false,
        }
    }

    fn print(&self) {
        self.print_rc(0);
    }


    fn print_rc(&self, depth: i32) {
        fn print_tab(count : i32) {
            for _ in 0..count {
                print!("\t");
            }
        }

        match &self.right {
            &None => {
                print_tab(depth + 1);
                println!("None @ {}",depth + 1);
            },
            &Some(ref right) => {
                right.print_rc(depth + 1);
            }
        }
        print_tab(depth);
        println!("{:?} @ {}", self.val, depth);
        match &self.left {
            &None => {
                print_tab(depth + 1);
                println!("None @ {}", depth + 1);
            },
            &Some(ref left) => {
                left.print_rc(depth + 1);
            }
        }
    }

}