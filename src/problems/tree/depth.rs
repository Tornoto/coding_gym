use std::{cell::RefCell, rc::Rc};

use super::binary_tree::TreeNode;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_depth = 0;
    let mut stack = vec![];
    let mut stack_support = vec![];
    if let Some(node) = root {
        stack.push(node);
    }

    while !stack.is_empty() {
        while let Some(node) = stack.pop() {
            if let Some(left) = node.borrow().left.clone() {
                stack_support.push(left);
            }
            if let Some(right) = node.borrow().right.clone() {
                stack_support.push(right);
            }
        }
        std::mem::swap(&mut stack, &mut stack_support);
        max_depth += 1;
    }

    max_depth
}

pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut min_depth = 0;
    let mut stack = vec![];
    let mut stack_support = vec![];
    if let Some(node) = root {
        stack.push(node);
    }

    while !stack.is_empty() {
        min_depth += 1;
        while let Some(node) = stack.pop() {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();

            if left.is_none() && right.is_none() {
                return min_depth;
            }

            if let Some(node_left) = left {
                stack_support.push(node_left);
            }

            if let Some(node_right) = right {
                stack_support.push(node_right);
            }
        }

        std::mem::swap(&mut stack, &mut stack_support);
    }
    min_depth
}

#[cfg(test)]
mod test {
    use crate::problems::tree::binary_tree::get_test_case;

    use super::*;

    #[test]
    fn test_max_depth() {
        let root = get_test_case();
        let depth = max_depth(root);
        println!("depth: {depth}");
    }

    #[test]
    fn test_min_depth() {
        let root = get_test_case();
        let depth = min_depth(root);
        println!("depth: {depth}");
    }
}
