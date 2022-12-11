use super::Tree;



enum Link<K,V> {
    Empty, 
    Filled(Box<TreeNode<K,V>>)
}

struct TreeNode<K,V> {
    key: K,
    value: V,
    left: Link<K,V>,
    right: Link<K,V>
}

pub struct BinaryTree<K,V> {
    entry: Link<K,V>
}

impl <K,V> BinaryTree<K,V> {
    pub fn new() -> Self {
        BinaryTree { entry: Link::Empty }
    }
}

impl <K: Ord, V> Tree<K, V> for BinaryTree<K, V> {
    
    fn put(&mut self, key: K, value: V) {
        let mut cursor = &mut self.entry;
        while let Link::Filled(n) = cursor {
            if n.key < key {
                cursor = &mut n.right;
            } else {
                cursor = &mut n.left;
            }
        }
        let _ = std::mem::replace(cursor, Link::Filled(Box::new(TreeNode::new(key,value))));
    }

    fn get(&self, key: K) -> Option<&V> {
        let mut cursor = &self.entry;
        while let Link::Filled(n) = cursor {
            match key.cmp(&n.key) {
                std::cmp::Ordering::Equal => {
                    return Some(&n.value);
                },
                std::cmp::Ordering::Greater => {
                    cursor = &n.right;
                },
                std::cmp::Ordering::Less => {
                    cursor = &n.left;
                }
            }
        }
        None
    }

    fn remove(&mut self, key: K) -> Option<V> {
        None        
    }
}

impl <K,V> TreeNode<K,V> {
    pub fn new(key: K,value: V) -> Self {
        TreeNode { key, value, left: Link::Empty, right: Link::Empty }
    }
}


#[test]
fn put_and_lookup() {
    let mut t = BinaryTree::new();
    for i in 0..5 {
        t.put(i, i.to_string());
    }
    for i in 0..5 {
        assert_eq!(t.get(i).unwrap().as_str(), i.to_string().as_str());
    }
}

