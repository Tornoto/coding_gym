use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct GraphNode {
    pub val: i32,
    pub neighbors: Vec<Rc<RefCell<GraphNode>>>,
}

impl GraphNode {
    pub fn new(val: i32) -> Self {
        Self {
            val: val,
            neighbors: vec![],
        }
    }
}

pub fn display(node: Option<Rc<RefCell<GraphNode>>>) {
    if node.is_none() {
        return;
    }

    let root = node.unwrap();
    let mut visited: HashSet<i32> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());

    let mut vals = vec![];
    while let Some(cur) = queue.pop_front() {
        let val = cur.borrow().val;
        if !visited.contains(&val) {
            vals.push(val);
            visited.insert(val);
            for neighbor in &cur.borrow().neighbors {
                queue.push_back(neighbor.clone());
            }
        }
    }
    println!("{:?}", vals);
}
