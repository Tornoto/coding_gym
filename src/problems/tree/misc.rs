use std::cell::RefCell;
use std::rc::Rc;

use super::binary_tree::TreeNode;

pub fn count_nodes_iter(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut count = 0;
    let mut stack = Vec::new();

    if let Some(node) = root {
        stack.push(node);
    }

    while let Some(node) = stack.pop() {
        count += 1;

        if let Some(right) = node.borrow().right.clone() {
            stack.push(right);
        }

        if let Some(left) = node.borrow().left.clone() {
            stack.push(left);
        }
    }

    count
}

pub fn count_nodes_rec(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();
        return match (left, right) {
            (Some(l), Some(r)) => 1 + count_nodes_rec(Some(l)) + count_nodes_rec(Some(r)),
            (Some(l), None) => 1 + count_nodes_rec(Some(l)),
            (None, Some(r)) => 1 + count_nodes_rec(Some(r)),
            _ => 1,
        };
    }
    0
}
