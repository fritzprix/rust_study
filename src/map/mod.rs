extern crate std;

pub mod tree;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::clone::Clone;
use std::hash::Hash;

pub trait Map<T>
{
    fn insert(&mut self, val: T);

    fn print(&self);

    fn has(&self, val: T) -> bool;

    fn remove_max(&mut self) -> Option<T>;

    fn remove_min(&mut self) -> Option<T>;

    fn remove(&mut self,val : T) -> bool;

    fn remove_all(&mut self) -> bool;

    fn join (self, another: Self) -> Self;

    fn size(&self) -> usize;
}

pub trait AMap<T>
    where T: Debug + Clone
{
    fn insert<K>(&mut self, key : K, val: T)  where K : Hash;

    fn print(&self);

    fn has<K>(&self, key: K) -> bool where K: Hash;

    fn remove<K>(&mut self,key : K) -> Option<T> where K: Hash;

    fn remove_all(&mut self) -> bool;

    fn join (self, another: Self) -> Self;

    fn size(&self) -> usize;
}


#[cfg(test)]
mod map_tests {

    use map::tree;
    use map::Map;

    #[test]
    fn binary_tree_basic_test() {
        let names = vec!["David","Alice","Bob","Jane","Andy","Charlie","Bruce"];
        let mut name_tree = names.clone().into_iter().collect::<tree::BinaryTree<&str>>();
        assert_eq!(name_tree.size(), names.len());

        for name in &names {
            // returned value of remove should not None
            assert_ne!(name_tree.remove(*name), None);
        }
        // after removing all the elements, the size should equal to 0
        assert_eq!(name_tree.size(), 0);

        let mut name_tree = names.clone().into_iter().collect::<tree::BinaryTree<&str>>();
        let size = name_tree.size();
        let mut max : Option<&str> = None;
        for _ in 0..size {
            match (max, name_tree.remove_max()) {
                (Some(ref max), Some(ref removed)) => {
                    assert!(!(max < removed));
                },
                (None, Some(ref removed)) => {
                    max = Some(removed);
                },
                (_, None) => {
                    assert!(false);
                }
            }
        }

        let mut name_tree = names.clone().into_iter().collect::<tree::BinaryTree<&str>>();
        let size = name_tree.size();
        let mut min : Option<&str> = None;
        for _ in 0..size {
            match (min, name_tree.remove_min()) {
                (Some(ref min), Some(ref removed)) => {
                    assert!(!(min > removed));
                },
                (None, Some(ref removed)) => {
                    min = Some(removed);
                },
                (_, None) => {
                    assert!(false);
                }
            }
        }

        assert_eq!(name_tree.size(),0);
    }
}
