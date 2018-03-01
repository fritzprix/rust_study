extern crate std;

pub fn new<T:std::cmp::PartialOrd + std::fmt::Debug + std::clone::Clone>() -> BinaryTree<T> {
    BinaryTree { entry: None }
}

pub struct BinaryTree<T: std::cmp::PartialOrd + std::fmt::Debug + std::clone::Clone> {
    entry: Option<Box<BinaryTreeNode<T>>>
}

impl<T: std::cmp::PartialOrd + std::fmt::Debug + std::clone::Clone> BinaryTree<T> {

    pub fn insert(&mut self, val: T) -> bool {
        let entry_node = &mut self.entry;
        match entry_node {
            &mut None => {
                *entry_node = Some(Box::new(BinaryTreeNode::new(val)));
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

    pub fn has(&self, val: T) -> bool {
        match &self.entry {
            &None => false,
            &Some(ref entry_node) => entry_node.has(val)
        }
    }


    pub fn remove_max(&mut self) -> Option<T> {
        let result: (Option<T>, Option<Box<BinaryTreeNode<T>>>, bool) = match &mut self.entry {
            &mut None => (None, None, false),
            &mut Some(ref mut entry_node) => entry_node.remove_max()
        };

        if let (_, ref replace, true) = result {
            self.entry.clone_from(replace);
        }
        result.0
    }

    pub fn remove_min(&mut self) -> Option<T> {
        let result: (Option<T>, Option<Box<BinaryTreeNode<T>>>, bool) = match &mut self.entry {
            &mut None => (None, None, false),
            &mut Some(ref mut entry_node) => entry_node.remove_min()
        };

        if let (_, ref replace, true) = result {
            self.entry.clone_from(replace);
        }
        result.0
    }

    pub fn remove(&mut self,val : T) -> bool {
        let result: (Option<T>, Option<Box<BinaryTreeNode<T>>>, bool) = match &mut self.entry {
            &mut None => (None, None, false),
            &mut Some(ref mut entry_node) => entry_node.remove(val)
        };
        if let (_, ref replace, true) = result {
            self.entry.clone_from(replace);
        }
        result.0 != None
    }

    pub fn remove_all(&mut self) -> bool {
        true
    }

    pub fn join (self, another: BinaryTree<T>) -> BinaryTree<T> {
        self
    }

    pub fn size(&self) -> usize {
        match &self.entry {
            &None => 0,
            &Some(ref entry_node) => entry_node.size()
        }
    }
}

#[derive(Clone)]
struct BinaryTreeNode<T: std::cmp::PartialOrd + std::fmt::Debug + std::clone::Clone> {
    val: T,
    left: Option<Box<BinaryTreeNode<T>>>,
    right: Option<Box<BinaryTreeNode<T>>>
}

enum Direction {
    Left,
    Right,
    Stop
}


impl<T: std::cmp::PartialOrd + std::fmt::Debug + std::clone::Clone> BinaryTreeNode<T> {
    fn new(init_val: T) -> BinaryTreeNode<T> {
        BinaryTreeNode { val: init_val, left: None, right: None }
    }

    fn remove_min(&mut self) -> (Option<T>, Option<Box<BinaryTreeNode<T>>>, bool) {
        let mut result = (None, None, false);
        if let Some(ref mut left_subtree) = self.left {
            result = left_subtree.remove_max();
        } else {
            return (Some(self.val.clone()), self.right.clone(), true);
        }

        if let (_, ref mut replace, ref mut updated @ true) = result {
            self.left.clone_from(replace);
            *updated = false;
        }
        result
    }

    fn take_min(&mut self) -> (Option<Box<BinaryTreeNode<T>>>, Option<Box<BinaryTreeNode<T>>>, bool) {
        let mut result = (None, None, false);
        if let Some(ref mut left_subtree) = self.left {
            result = left_subtree.take_min();
        } else {
            return (Some(Box::new(self.clone())), self.right.clone(), true);
        }

        if let (_, ref replace, ref mut updated @ true) = result {
            self.left.clone_from(replace);
            *updated = false;
        }
        result
    }

    fn remove_max(&mut self) -> (Option<T>, Option<Box<BinaryTreeNode<T>>>, bool) {
        let mut result = (None, None, false);
        if let Some(ref mut right_subtree) = self.right {
            result = right_subtree.remove_max();
        } else {
            return (Some(self.val.clone()), self.left.clone(), true);
        }

        if let (_, ref mut replace, ref mut updated @ true) = result {
            self.right.clone_from(replace);
            *updated = false;
        }
        result
    }

    fn take_max(&mut self) -> (Option<Box<BinaryTreeNode<T>>>, Option<Box<BinaryTreeNode<T>>>, bool) {
        let mut result = (None, None, false);
        if let Some(ref mut right_subtree) = self.right {
            result = right_subtree.take_max();
        } else {
            return (Some(Box::new(self.clone())), self.left.clone(), true);
        }

        if let (_, ref replace, ref mut updated @ true) = result {
            self.right.clone_from(replace);
            *updated = false;
        }
        result
    }

    fn remove(&mut self, val : T) -> (Option<T>, Option<Box<BinaryTreeNode<T>>>, bool) {
        let mut result = (None, None, false);
        match self.val.partial_cmp(&val).expect("") {
            std::cmp::Ordering::Greater => {
                result = match &mut self.left {
                    &mut Some(ref mut left) => left.remove(val),
                    &mut None => (None, None, false)
                };
                if let (_, ref replace,  replace_required @ true) = result {

                }
                result
            },
            std::cmp::Ordering::Less => {
                result = match &mut self.right {
                    &mut Some(ref mut right) => right.remove(val),
                    &mut None => (None, None, false)
                };
                if let (_, ref replace,  replace_required @ true) = result {
                }
                result
            },
            std::cmp::Ordering::Equal => {
                return match (&mut self.left, &mut self.right) {
                    (&mut None, &mut None) => (Some(self.val.clone()), None, true),
                    (&mut Some(ref mut left),_) => {
                        match left.take_max() {
                            (ref taken, ref replace, is_replace_required @ true) => (Some(self.val.clone()), taken.clone(), true),
                            (ref taken, _, false) => (Some(self.val.clone()), taken.clone(), true),
                            _ => (None, None, false)
                        }
                    },
                    (&mut None, &mut Some(ref mut right)) => {
                        match right.take_min() {
                            (ref taken, ref replace, is_replace_required @ true) => (Some(self.val.clone()), taken.clone(), true),
                            (ref taken, _, false) => (Some(self.val.clone()), taken.clone(), true),
                            _ => (None, None, false)
                        }
                    }
                };

            }
        }

    }

    fn insert(&mut self, val: T) -> bool {
        let target = if self.val > val { &mut self.left } else { &mut self.right };
        match target {
            &mut None => {
                *target = Some(Box::new(BinaryTreeNode { val, left: None, right: None }));
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

    fn has(&self, new_val: T) -> bool {
        match self {
            &BinaryTreeNode { val: ref cur_val, left: _, ref right } if *cur_val < new_val => {
                match right {
                    &None => false,
                    &Some(ref right_child) => right_child.has(new_val)
                }
            },
            &BinaryTreeNode { val: ref cur_val, ref left, right: _ } if *cur_val > new_val => {
                match left {
                    &None => false,
                    &Some(ref left_child) => left_child.has(new_val)
                }
            },
            &BinaryTreeNode { val: ref cur_val, left: _, right: _ } if *cur_val == new_val => true,
            _ => false,
        }
    }

    fn print(&self) {
        self.print_rc(0);
    }


    fn print_rc(&self, depth: i32) {
        fn print_tab(count: i32) {
            for _ in 0..count {
                print!("\t");
            }
        }

        match &self.right {
            &None => {
                print_tab(depth + 1);
                println!("None @ {}", depth + 1);
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