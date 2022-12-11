use super::Queue;
use std::{prelude::v1::*, rc::Rc, ptr::null};

struct ListNode<T> {
    value: T,
    next: Link<T>,
}

enum Link<T> {
    Empty,
    Filled(Box<ListNode<T>>),
}

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {

    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn iter(&self) -> ListIterator<T> {
        ListIterator { cursor: &self.head }
    }
}

impl <T> Queue<T> for List<T> {
    
    fn enqueue(&mut self, item: T) {

        let mut cursor: &mut Link<T> = &mut self.head;
        while let Link::Filled(next, ..) = cursor {
            cursor = &mut next.next;
        }
        let _ = std::mem::replace(cursor, Link::Filled(Box::new(ListNode::new(item))));
    }

    fn dequeue(&mut self) -> Option<T> {
        let head = std::mem::replace(&mut self.head, Link::Empty);
        match head {
            Link::Empty => None,
            Link::Filled(n) => {
                let _ = std::mem::replace(&mut self.head, n.next);
                Some(n.value)
            }
        }
    }
}

pub struct ListIterator<'a, T> {
    cursor: &'a Link<T>
}

impl <'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.cursor {
            Link::Empty => None,
            Link::Filled(n) => {
                self.cursor = &n.next;
                Some(&n.value)
            }
        }
    }
}

impl <T> ListNode<T> {
    fn new(value: T) -> Self {
        ListNode { value, next: Link::Empty }
    }
}


#[test]
fn symmetric_enqueue_dequeue() {
    let mut l = List::new();
    for i in 0..5 {
        l.enqueue(i.to_string());
    }

    for i in 0..5 {
        assert_eq!(l.dequeue(), Some(i.to_string()));
    }
}