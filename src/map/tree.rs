use std;
use super::Map;




pub struct BinaryTree<T>
    where T: std::cmp::PartialOrd + std::fmt::Debug + std::clone::Clone
{
    entry: Option<Box<BinaryTreeNode<T>>>,
}

impl<T> BinaryTree<T>
    where T: std::cmp::PartialOrd + std::fmt::Debug + std::clone::Clone
{
    pub fn new() -> Self
    {
        BinaryTree { entry: None  }
    }
}

impl<A> std::iter::FromIterator<A> for BinaryTree<A>
    where A: std::cmp::PartialOrd + std::fmt::Debug + std::clone::Clone
{

    fn from_iter<T: IntoIterator<Item=A>>(iter: T) -> Self {
        let into = iter.into_iter();
        let mut new_tree  = BinaryTree::new();
        for i in into {
            new_tree.insert(i);
        }
        new_tree
    }
}

impl<T> std::iter::Iterator for  BinaryTree<T>
    where T: std::cmp::PartialOrd + std::fmt::Debug + std::clone::Clone
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.remove_min()
    }
}



impl<T> Map<T> for BinaryTree<T>
    where T: std::cmp::PartialOrd + std::fmt::Debug + std::clone::Clone
{

    fn insert(&mut self, val: T) -> bool {
        let entry_node = &mut self.entry;
        match entry_node {
            &mut None => {
                *entry_node = Some(Box::new(BinaryTreeNode::new(val)));
                true
            },
            &mut Some(ref mut entry_node) => entry_node.insert(val)
        }
    }

    fn print(&self) {
        match &self.entry {
            &None => println!("Empty"),
            &Some(ref entry) => entry.print()
        }
    }

    fn has(&self, val: T) -> bool {
        match &self.entry {
            &None => false,
            &Some(ref entry_node) => entry_node.has(val)
        }
    }


    fn remove_max(&mut self) -> Option<T> {
        let result: (Option<T>, Option<Box<BinaryTreeNode<T>>>, bool) = match &mut self.entry {
            &mut None => (None, None, false),
            &mut Some(ref mut entry_node) => entry_node.remove_max()
        };

        if let (_, ref replace, true) = result {
            self.entry.clone_from(replace);
        }
        result.0
    }

    fn remove_min(&mut self) -> Option<T> {
        let result: (Option<T>, Option<Box<BinaryTreeNode<T>>>, bool) = match &mut self.entry {
            &mut None => (None, None, false),
            &mut Some(ref mut entry_node) => entry_node.remove_min()
        };

        if let (_, ref replace, true) = result {
            self.entry.clone_from(replace);
        }
        result.0
    }

    fn remove(&mut self,val : T) -> Option<T> {
        let result: (Option<T>, Option<Box<BinaryTreeNode<T>>>, bool) = match &mut self.entry {
            &mut None => (None, None, false),
            &mut Some(ref mut entry_node) => entry_node.remove(val)
        };

        if let (_, ref replace, true) = result {
            self.entry.clone_from(replace);
        }
        result.0
    }

    fn remove_all(&mut self) -> bool {
        self.entry = None;
        true
    }

    fn join (mut self, another: Self) -> Self {
        for i in another {
            self.insert(i);
        }
        self
    }

    fn size(&self) -> usize {
        match &self.entry {
            &None => 0,
            &Some(ref entry_node) => entry_node.size()
        }
    }
}

#[derive(Clone, Debug)]
struct BinaryTreeNode<T>
    where T: std::cmp::PartialOrd + std::fmt::Debug + std::clone::Clone
{
    val: T,
    left: Option<Box<BinaryTreeNode<T>>>,
    right: Option<Box<BinaryTreeNode<T>>>,
}

impl<T> BinaryTreeNode<T>
    where T: std::cmp::PartialOrd + std::fmt::Debug + std::clone::Clone {
    fn new(init_val: T) -> BinaryTreeNode<T> {
        BinaryTreeNode { val: init_val, left: None, right: None }
    }

    fn remove_min(&mut self) -> (Option<T>, Option<Box<BinaryTreeNode<T>>>, bool) {
        let mut result;
        if let Some(ref mut left_subtree) = self.left {
            result = left_subtree.remove_min();
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
        let mut result;
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
        let mut result;
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
        let mut result;
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
        match self.val.partial_cmp(&val).expect("") {
            std::cmp::Ordering::Greater => {
                let mut result = match &mut self.left {
                    &mut Some(ref mut left) => left.remove(val),
                    &mut None => (None, None, false)
                };
                if let (_, ref replace,  ref mut replace_required @ true) = result {
                    *replace_required = false;
                    self.left.clone_from(replace);
                }
                result
            },
            std::cmp::Ordering::Less => {
                let mut result = match &mut self.right {
                    &mut Some(ref mut right) => right.remove(val),
                    &mut None => (None, None, false)
                };
                if let (_, ref replace,  ref mut replace_required @ true) = result {
                    *replace_required = false;
                    self.right.clone_from(replace);
                }
                result
            },
            std::cmp::Ordering::Equal => {
                return match (&mut self.left, &mut self.right) {
                    (&mut None, &mut None) => (Some(self.val.clone()), None, true),
                    (&mut Some(ref mut left),ref mut right) => {
                        match left.take_max() {
                            (Some(ref mut taken), ref replace, ref update_required) => {
                                if *update_required {
                                    taken.left = replace.clone();
                                } else {
                                    taken.left = Some(left.clone());
                                }
                                taken.right = right.clone();
                                (Some(self.val.clone()), Some(taken.clone()), true)
                            },
                            _=> (None, None, false)
                        }
                    },
                    (ref mut left, &mut Some(ref mut right)) => {
                        match right.take_min() {
                            (Some(ref mut taken), ref replace, ref update_required) => {
                                if *update_required {
                                    taken.right = replace.clone();
                                } else {
                                    taken.right = Some(right.clone());
                                }
                                taken.left = left.clone();
                                (Some(self.val.clone()), Some(taken.clone()), true)
                            },
                            _=> (None, None, false)
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