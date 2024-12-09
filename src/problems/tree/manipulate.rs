use std::{cell::RefCell, rc::Rc};

use super::binary_tree::TreeNode;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut stack = vec![];
    if let Some(node) = root.clone() {
        stack.push(node);
    }

    while !stack.is_empty() {
        if let Some(node) = stack.pop() {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            node.borrow_mut().left = right.clone();
            node.borrow_mut().right = left.clone();
            if left.is_some() {
                stack.push(left.unwrap());
            }
            if right.is_some() {
                stack.push(right.unwrap());
            }
        }
    }

    root
}

pub fn invert_tree_rec(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root.clone() {
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();

        node.borrow_mut().left = right.clone();
        node.borrow_mut().right = left.clone();

        if left.is_some() {
            invert_tree_rec(left);
        }

        if right.is_some() {
            invert_tree_rec(right);
        }
    }

    root
}
