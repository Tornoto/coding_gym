use std::{cell::RefCell, rc::Rc};

use super::binary_tree::TreeNode;

/// https://leetcode.com/problems/symmetric-tree/description/
pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut stack = vec![];
    let mut stack_mirror = vec![];

    stack.push(root.clone());
    stack_mirror.push(root.clone());

    while !stack.is_empty() && !stack_mirror.is_empty() {
        if let (Some(node), Some(node_mirror)) = (stack.pop(), stack_mirror.pop()) {
            match (node, node_mirror) {
                (Some(left), Some(right)) => {
                    if left.borrow().val == right.borrow().val {
                        stack.push(left.borrow().left.clone());
                        stack.push(left.borrow().right.clone());

                        // push nodes into stack_mirror with different order
                        stack_mirror.push(right.borrow().left.clone());
                        stack_mirror.push(right.borrow().right.clone());
                    } else {
                        return false;
                    }
                }
                (None, None) => continue,
                _ => return false,
            }
        }
    }

    true
}

pub fn is_symmetric_rec(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_symmetric_rec_sub(root.clone(), root.clone())
}

fn is_symmetric_rec_sub(
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (left, right) {
        (Some(node_left), Some(node_right)) => {
            if node_left.borrow().val == node_right.borrow().val {
                is_symmetric_rec_sub(
                    node_left.borrow().left.clone(),
                    node_right.borrow().right.clone(),
                ) && is_symmetric_rec_sub(
                    node_left.borrow().right.clone(),
                    node_right.borrow().left.clone(),
                )
            } else {
                false
            }
        }
        (None, None) => true,
        _ => false,
    }
}
