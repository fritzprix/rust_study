use std::rc::Rc;


struct ListNode<T> {
    next: Option<Box<ListNode<T>>>,
    value: T
}

struct List<T> {
    entry: Option<Box<ListNode<T>>>
}

trait Queue<T> {
    fn enqueu(&mut self,value: T) -> usize;
    fn dequeu(&mut self) -> Option<T>;
}

impl <T> List<T> {
    fn new() -> Self {
        List { entry: None }
    }


    fn len(&self) -> usize {
        let mut entry = &self.entry;
        let mut size = 0usize;
        while let Some(a) = entry {
            entry = &a.next;
            size += 1;
        }
        size
    }
}

impl <T> ListNode<T> {
    fn new(value: T) -> Self {
        ListNode { next: None, value }
    }

    fn put_tail(&mut self, value: T) -> usize {
        match &mut self.next {
            Some(next) => {
                next.put_tail(value) + 1
            },
            None => {
                self.next = Some(Box::new(ListNode::new(value)));
                1
            },
        }
    }

    fn unwrap(self) -> T {
        self.value
    }
}

impl <T> Queue<T> for List<T> {
    
    fn dequeu(&mut self) -> Option<T> {
        let head = std::mem::replace(&mut self.entry, None);
        match head {
            Some(node) => {
                let a = match node.unwrap() {
                    ListNode{next, value} => {},
                    _ => {}
                };
                let _ = std::mem::replace(&mut self.entry, node.next);
                Some(node.unwrap())
            },
            None => None
        }
    }

    fn enqueu(&mut self,value: T) -> usize {
        match &mut self.entry {
            Some(entry) => {
                entry.put_tail(value) + 1
            },
            _ => {
                self.entry = Some(Box::new(ListNode::new(value)));
                1
            },
        }
    }
}


#[test]
fn add_item_into_list() {
    let mut ls = List::<u8>::new();

    for v in 0..100 {
        ls.enqueu(v);
    }
    assert_eq!(ls.len(), 100);
    assert_eq!(ls.dequeu(), Some(0));
}
