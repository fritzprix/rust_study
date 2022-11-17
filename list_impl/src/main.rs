

mod listrc;
#[derive(Debug)]
struct Node<T> {
    next: Option<Box<Node<T>>>,
    value: T
}

#[derive(Debug)]
struct List<T> {
    entry: Option<Node<T>>
}

impl <T> List<T> {
    fn new() -> Self {
        List { entry: None }
    }

    fn push(&mut self, value: T) -> usize {
        match &mut self.entry {
            Some(entry) => entry.push(value) + 1,
            _ => {
                self.entry = Some(Node::new(value));
                1
            },
        }
    }

    fn pop(&mut self) -> Option<T> {
        None
    }

    fn len(&self) -> usize {
        0
    }
}

impl <T> Node<T> {
    fn new(value: T) -> Self {
        Node { next: None, value }
    }

    fn push(&mut self, value: T) -> usize {
        match &mut self.next {
            Some(next) => next.push(value) + 1,
            _ => {
                self.next = Some(Box::new(Node::new(value)));
                1
            },
        }
    }

    fn pop(&mut self) -> Option<T> {
        None
    }
}





fn main() {
    let mut l = List::<u8>::new();
    println!("push(2) {}",l.push(2));
    println!("push(3) {}",l.push(3));
    println!("List : {:?}", l);
}
