use super::Map;
use super::AMap;

use std::fmt::Debug;
use std::cmp::Ordering::*;
use std::clone::Clone;
use std::iter::{FromIterator,Iterator};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

type Link<T> = Option<Box<BinaryTreeNode<T>>>;
pub struct BinaryTree<T: Ord + Debug + Copy>
{
    root: Link<T>,
}

impl<T: Ord + Debug + Copy> BinaryTree<T>
{
    pub fn new() -> Self
    {
        BinaryTree { root: None  }
    }
}

impl<A: Ord + Debug + Copy> FromIterator<A> for BinaryTree<A>
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

impl<T: Ord + Debug + Copy> Iterator for  BinaryTree<T>
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.remove_min()
    }
}



impl<T: Ord + Debug + Copy> Map<T> for BinaryTree<T>
{

    fn insert(&mut self, val: T) {
        match &mut self.root {
            &mut Some(ref mut root) => root.insert(val),
            root => *root = Some(Box::new(BinaryTreeNode::new(val)))
        }
    }

    fn remove(&mut self, val : T) -> bool {
        let rmv = match &mut self.root {
            &mut None => return false,
            some => BinaryTreeNode::locate(some, val)
        };

        let rmv_val = match rmv {
            &mut None => return false,
            &mut Some(ref mut target) if target.left.is_some() => BinaryTreeNode::take_right_most(&mut target.left),
            &mut Some(ref mut target) if target.right.is_some() => BinaryTreeNode::take_left_most(&mut target.right),
            _ => None
        };

        match (rmv, rmv_val) {
            (_rmv, None) => {
                *_rmv = None;
                true
            },
            (&mut Some(ref mut _rm_node), Some(_rv)) => {
                _rm_node.val = _rv;
                true
            },
            _ => false
        }
    }

    fn remove_max(&mut self) -> Option<T> {
        match &mut self.root {
            &mut None => None,
            some => BinaryTreeNode::take_right_most(some)
        }
    }

    fn remove_min(&mut self) -> Option<T> {
        match &mut self.root {
            &mut None => None,
            some => BinaryTreeNode::take_left_most(some)
        }
    }

    fn print(&self) {
        println!();
        match self.root {
            Some(ref root_node) => root_node.print(0),
            _ => println!("Empty")
        }
        println!();
    }


    fn has(&self, val: T) -> bool {
        match &self.root {
            &None => false,
            &Some(ref entry_node) => entry_node.has(val)
        }
    }


    fn remove_all(&mut self) -> bool {
        self.root = None;
        true
    }

    fn join (mut self, another: Self) -> Self {
        for i in another {
            self.insert(i);
        }
        self
    }

    fn size(&self) -> usize {
        match &self.root {
            &None => 0,
            &Some(ref entry_node) => entry_node.size()
        }
    }
}

#[derive(Debug)]
struct BinaryTreeNode<T: Ord + Debug + Copy>
{
    val: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Ord + Debug + Copy> BinaryTreeNode<T>
{

    fn new(val : T) -> BinaryTreeNode<T>
    {
        BinaryTreeNode {val , left : None, right : None}
    }

    fn print(&self, depth : u32) {
        if let Some(ref right) = self.right {
            right.print(depth + 1);
        }
        for _ in 0 .. depth {
            print!("\t");
        }
        println!("{:?} @ {}",self.val, depth);
        if let Some(ref left) = self.left {
            left.print(depth + 1);
        }
    }

    fn insert(&mut self, val : T)  {
        match match self.val > val {
            true => &mut self.left,
            false => &mut self.right
        } {
            &mut Some(ref mut target) => target.insert(val),
            _none => *_none = Some(Box::new(BinaryTreeNode::new(val)))
        }
    }

    fn take_right_most(link : &mut Link<T>) -> Option<T> {
        let right_most = BinaryTreeNode::right_leaf(link);
        let replace = match right_most {
            &mut Some(ref mut rm) if rm.left.is_some() => (rm.left.take(), Some(rm.val)),
            &mut Some(ref mut rm) => (None, Some(rm.val)),
            _ => (None, None)
        };
        if let (_replace, Some(_)) = replace {
            *right_most = _replace;
        }
        replace.1
    }

    fn right_leaf(link : &mut Link<T>) -> &mut Link<T> {
        match link {
            &mut Some(ref mut current) if current.right.is_some() => BinaryTreeNode::right_leaf(&mut current.right),
            other => return other
        }
    }

    fn take_left_most(link : &mut Link<T>) -> Option<T> {
        let left_most = BinaryTreeNode::left_leaf(link);
        let replace = match left_most {
            &mut Some(ref mut rm) if rm.right.is_some() => (rm.right.take(), Some(rm.val)),
            &mut Some(ref mut rm) => (None, Some(rm.val)),
            _ => (None,None)
        };

        if let (_replace, Some(_)) = replace {
            *left_most = _replace;
        }
        replace.1
    }


    fn left_leaf(link : &mut Link<T>) -> &mut Link<T> {
        match link {
            &mut Some(ref mut current) if current.left.is_some() => BinaryTreeNode::left_leaf(&mut current.left),
            other => return other
        }
    }

    fn locate(link : &mut Link<T>, val : T) -> &mut Link<T> {
        match link {
            &mut None => link,
            some => {
                let next = match match some {
                    &mut Some(ref cur) => cur.val.cmp(&val),
                    _ => return some
                } {
                    Greater => match some {
                        &mut Some(ref mut _cur) => &mut _cur.left,
                        _ => return some
                    },
                    Less => match some {
                        &mut Some(ref mut _cur) => &mut _cur.right,
                        _ => return some
                    },
                    Equal => return some
                };
                BinaryTreeNode::locate(next, val)
            },
        }
    }


    fn size(&self) -> usize {
        let mut size = 0;
        if let Some(ref right) = self.right {
            size += right.size();
        }
        if let Some(ref left) = self.left {
            size += left.size();
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
}

#[derive(Clone)]
pub struct ABinaryTreeNode<V> where V: Debug + Clone
{
    key :u64,
    val : V,
    right : Option<Box<ABinaryTreeNode<V>>>,
    left :Option<Box<ABinaryTreeNode<V>>>
}

pub struct ABinaryTree<V> where V: Debug + Clone
{
    entry: Option<Box<ABinaryTreeNode<V>>>,
}

impl<V> ABinaryTree<V> where V: Debug + Clone
{
    pub fn new() -> Self {
        ABinaryTree { entry: None }
    }
}

impl<V> ABinaryTree<V> where V: Debug + Clone
{
    fn hash<K>(key: K) -> u64 where K: Hash {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }
}

impl<V> AMap<V> for ABinaryTree<V> where V: Debug + Clone
{

    fn insert<K>(&mut self, key: K, val: V) where K: Hash {
        let key_hash = Self::hash(key);
        match self.entry {
            Some(ref mut entry_node) => entry_node.insert(key_hash, val),
            None => self.entry = Some(Box::new(ABinaryTreeNode::new(key_hash, val)))
        }
    }

    fn print(&self) {
        if let Some(ref entry_node) = self.entry {
            entry_node.print();
        }
    }

    fn has<K>(&self, key: K) -> bool where K: Hash {
        let key_hash = Self::hash(key);
        match self.entry {
            Some(ref entry_node) => entry_node.has(key_hash),
            None => false
        }
    }

    fn remove<K>(&mut self, key: K) -> Option<V> where K: Hash {
        let key_hash = Self::hash(key);
        None
    }

    fn remove_all(&mut self) -> bool {
        unimplemented!()
    }

    fn join(self, another: Self) -> Self {
        unimplemented!()
    }

    fn size(&self) -> usize {
        unimplemented!()
    }

}

impl <V> ABinaryTreeNode<V> where V:Debug + Clone {

    fn new(key : u64, val : V) -> Self {
        ABinaryTreeNode {key, val, left: None, right: None}
    }

    fn insert(&mut self, key: u64, val :V)
    {

        match match self.key > key {
            true => &mut self.left,
            false => &mut self.right
        } {
            &mut Some(ref mut next) => next.insert(key, val),
            _next => *_next = Some(Box::new(ABinaryTreeNode::new(key,val)))
        }
    }

    fn locate(link : &mut Option<Box<ABinaryTreeNode<V>>>, key: u64) -> &mut Option<Box<ABinaryTreeNode<V>>>
    {
        link
    }



    fn print(&self) {
        self.print_rc(0);
    }

    fn has(&self, key : u64) -> bool {
        if self.key == key {
            return true;
        } else if self.key > key {
            return match self.left {
                Some(ref left) => left.has(key),
                None => false
            };
        } else {
            return match self.right {
                Some(ref right) => right.has(key),
                None => false
            };
        }
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
