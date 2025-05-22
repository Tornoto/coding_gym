#![allow(unused)]
struct MyLinkedList {
    root: Option<Box<Node>>,
}

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList { root: None }
    }

    fn get(&self, index: i32) -> i32 {
        let mut cur = &self.root;

        for _ in 0..index {
            match cur {
                Some(node) => cur = &node.next,
                None => return -1,
            }
        }

        match cur {
            Some(node) => node.val,
            None => -1,
        }
    }

    fn add_at_head(&mut self, val: i32) {
        let new_head = Node {
            val: val,
            next: self.root.take(),
        };
        self.root = Some(Box::new(new_head));
    }

    fn add_at_tail(&mut self, val: i32) {
        let tail = Node {
            val: val,
            next: None,
        };

        if self.root.is_none() {
            self.root = Some(Box::new(tail));
            return;
        }

        let mut cur = self.root.as_mut();
        while let Some(node) = cur {
            match node.next {
                Some(_) => cur = node.next.as_mut(),
                None => {
                    node.next = Some(Box::new(tail));
                    break;
                }
            }
        }
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index < 0 {
            return;
        }
        if index == 0 {
            self.add_at_head(val);
            return;
        }

        let mut cur = self.root.as_mut();
        for _ in 0..index - 1 {
            match cur {
                Some(node) => cur = node.next.as_mut(),
                None => return,
            }
        }

        if let Some(node) = cur {
            let new_node = Box::new(Node {
                val,
                next: node.next.take(),
            });
            node.next = Some(new_node);
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if index < 0 {
            return;
        }
        if index == 0 {
            self.root = self.root.take().and_then(|head| head.next);
            return;
        }

        let mut cur = self.root.as_mut();
        for _ in 0..index - 1 {
            match cur {
                Some(node) => cur = node.next.as_mut(),
                None => return,
            }
        }

        if let Some(node) = cur {
            node.next = node.next.take().and_then(|next| next.next);
        }
    }

    fn display(&self) {
        let mut cur = &self.root;
        while let Some(node) = cur {
            print!("{} -> ", node.val);
            cur = &node.next;
        }
    }
}

#[test]
fn test() {
    let mut root = MyLinkedList::new();
    root.add_at_head(1);
    root.add_at_tail(3);
    root.display();
    println!("\n----");
    root.add_at_index(1, 2);
    root.display();
    root.delete_at_index(1);
    println!("\n----");
    root.display();
}
