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

pub fn max_depth_rec(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    max_depth_rec_sub(root, 0)
}

fn max_depth_rec_sub(root: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
    let mut depth = depth;
    if let Some(node) = root {
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();
        match (left, right) {
            (Some(node_left), Some(node_right)) => {
                depth = std::cmp::max(
                    max_depth_rec_sub(Some(node_left), depth + 1),
                    max_depth_rec_sub(Some(node_right), depth + 1),
                )
            }
            (Some(node_left), None) => depth = max_depth_rec_sub(Some(node_left), depth + 1),
            (None, Some(node_right)) => depth = max_depth_rec_sub(Some(node_right), depth + 1),
            (None, None) => depth += 1,
        }
    }
    depth
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

pub fn min_depth_rec(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    min_depth_rec_sub(root)
}

fn min_depth_rec_sub(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();

        return match (left, right) {
            (Some(node_left), Some(node_right)) => {
                1 + std::cmp::min(
                    min_depth_rec_sub(Some(node_left)),
                    min_depth_rec_sub(Some(node_right)),
                )
            }
            (Some(node_left), None) => 1 + min_depth_rec_sub(Some(node_left)),
            (None, Some(node_right)) => 1 + min_depth_rec_sub(Some(node_right)),
            (None, None) => 1,
        };
    }
    0
}

#[cfg(test)]
mod test {
    use crate::problems::tree::binary_tree::get_test_case;

    use super::*;

    #[test]
    fn test_max_depth() {
        let root = get_test_case();
        let depth1 = max_depth(root.clone());
        let depth2 = max_depth_rec(root.clone());
        assert_eq!(depth1, depth2)
    }

    #[test]
    fn test_min_depth() {
        let root = get_test_case();
        let depth1 = min_depth(root.clone());
        let depth2 = min_depth_rec(root.clone());
        assert_eq!(depth1, depth2);
    }
}
