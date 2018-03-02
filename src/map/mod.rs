pub mod binary_tree;

extern crate std;

pub trait Map<T>
    where T: std::cmp::PartialOrd + std::fmt::Debug + std::clone::Clone {
    fn insert(&mut self, val: T) -> bool;

    fn print(&self);

    fn has(&self, val: T) -> bool;


    fn remove_max(&mut self) -> Option<T>;

    fn remove_min(&mut self) -> Option<T>;

    fn remove(&mut self,val : T) -> Option<T>;

    fn remove_all(&mut self) -> bool;

    fn join (self, another: Self) -> Self;

    fn size(&self) -> usize;
}

#[cfg(test)]
mod map_tests {

    use map::binary_tree;

    #[test]
    fn binary_tree_basic_test() {
        let names = vec!["David","Alice","Bob","Jane","Andy","Charlie","Bruce"];
        let mut name_tree = binary_tree::new();
        for name in &names {
            name_tree.insert(*name);
        }

        assert_eq!(name_tree.size(), names.len());
        for i in 0..names.len() {
            match name_tree.remove_min() {
                Some(name) => assert_eq!(name_tree.has(name), false),
                None => assert!(false)
            }
        }

        assert_eq!(name_tree.size(), 0);

        for name in &names {
            name_tree.insert(*name);
        }

        assert_eq!(name_tree.size(), names.len());
        for i in 0..names.len() {
            match name_tree.remove_max() {
                Some(name) => assert_eq!(name_tree.has(name), false),
                None => assert!(false)
            }
        }

        assert_eq!(name_tree.size(), 0);
    }
}
