pub mod binary_tree;

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
