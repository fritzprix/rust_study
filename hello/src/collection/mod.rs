pub mod list;
pub mod binarytree;

pub trait Tree<K,V> where K: Ord {
    fn put(&mut self, key: K, value: V);
    fn get(&self, key: K) -> Option<&V>;
    fn remove(&mut self, key: K) -> Option<V>;
}

pub trait Queue<T> {    
    fn enqueue(&mut self, item: T);
    fn dequeue(&mut self) -> Option<T>;
}

pub trait Stack<T> {
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
}