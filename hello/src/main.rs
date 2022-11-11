
use std::fmt::Debug;

struct ListNode<T> {
    next: Option<Box<ListNode<T>>>,
    value: T,
}


struct List<T> {
    head: Option<Box<ListNode<T>>>,
}

impl <T> List<T> where T: ToString + Sized {
    fn new() -> Self {
    }
}

impl <T> ListNode<T> where T: ToString + Sized {
    fn new(value: T) -> Self {
        ListNode { next: None, value }
    }

    fn value(&self) -> &T {
        &self.value
    }
}

impl <T> Debug for List<T> where T: ToString + Sized {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = "[".to_string();
        let mut cur = &self.head;
        while let Some(n) = cur {
            res.push_str(n.value().to_string().as_str());
            cur = &n.next;
            if cur.is_some() {
                res.push_str(", ")
            }
        }
        res.push_str("]");
        f.write_str(res.as_str())
    }
}

impl <T> List<T> where T: ToString + Sized {
    fn enqueue(&mut self, value: T) {
        let mut cur = &mut self.head;
        loop {
            match cur {
                Some(c) => {
                    cur = &mut c.next
                },
                _ => {
                    let _ = std::mem::replace(cur, Some(Box::new(ListNode::new(value))));
                    return;
                }
            };
        }
    }

    fn dequeue(&mut self) -> Option<T> {
        let head = std::mem::replace(&mut self.head, None);
        if let Some(n) = head {
            let _ = std::mem::replace(&mut self.head, n.next);
            Some(n.value)
        } else {
            None
        }
    }
}

fn main() {
    let mut my_list = List::<u32>::new();
    for i in 0..5 {
        my_list.enqueue(i)
    }
    println!("{:?}", my_list);
    assert_eq!(my_list.dequeue(), Some(0u32));
    println!("{:?}", my_list);

}

