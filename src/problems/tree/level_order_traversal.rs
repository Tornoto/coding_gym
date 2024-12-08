#![allow(unused)]
use std::{cell::RefCell, rc::Rc};

use super::binary_tree::TreeNode;

/// https://leetcode.com/problems/binary-tree-level-order-traversal/description/
fn level_order_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut stack = vec![];
    let mut stack_support = vec![];
    if let Some(node) = root {
        stack.push(node);
    }

    while !stack.is_empty() {
        let mut level_result = vec![];
        while let Some(node) = stack.pop() {
            level_result.push(node.borrow().val);
            if let Some(left) = node.borrow().left.clone() {
                stack_support.push(left);
            }
            if let Some(right) = node.borrow().right.clone() {
                stack_support.push(right);
            }
        }
        while let Some(node) = stack_support.pop() {
            stack.push(node);
        }
        result.push(level_result);
    }

    result
}

/// https://leetcode.cn/problems/binary-tree-level-order-traversal-ii/description/
fn level_order_traversal_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut result = level_order_traversal(root);
    result.reverse();
    result
}

/// https://leetcode.cn/problems/binary-tree-right-side-view/description/
/// push the last element of each level into result
pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];
    let mut stack = vec![];
    let mut stack_support = vec![];
    if let Some(node) = root {
        stack.push(node);
    }

    while !stack.is_empty() {
        let mut level_result = vec![];
        while let Some(node) = stack.pop() {
            level_result.push(node.borrow().val);
            if let Some(left) = node.borrow().left.clone() {
                stack_support.push(left);
            }
            if let Some(right) = node.borrow().right.clone() {
                stack_support.push(right);
            }
        }
        while let Some(node) = stack_support.pop() {
            stack.push(node);
        }
        if let Some(val) = level_result.last() {
            result.push(*val);
        }
    }

    result
}

/// https://leetcode.cn/problems/average-of-levels-in-binary-tree/description/
pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
    let result = level_order_traversal(root);
    result
        .into_iter()
        .map(|arr: Vec<i32>| (arr.iter().sum::<i32>()) as f64 / arr.len() as f64)
        .collect()
}

/// same solution with different result type
fn level_order_traversal2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];
    let mut stack = vec![];
    let mut stack_support = vec![];
    if let Some(node) = root {
        stack.push(node);
    }

    while !stack.is_empty() {
        while let Some(node) = stack.pop() {
            result.push(node.borrow().val);
            if let Some(left) = node.borrow().left.clone() {
                stack_support.push(left);
            }
            if let Some(right) = node.borrow().right.clone() {
                stack_support.push(right);
            }
        }
        while let Some(node) = stack_support.pop() {
            stack.push(node);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use crate::problems::tree::binary_tree::get_test_case;

    use super::*;

    #[test]
    fn test_level_order_traversal() {
        let root = get_test_case();
        let result = level_order_traversal(root);
        println!("res: {:?}", result);
    }

    #[test]
    fn test_right_side_view() {
        let root = get_test_case();
        let result = right_side_view(root);
        println!("res: {:?}", result);
    }

    #[test]
    fn test_average_of_levels() {
        let root = get_test_case();
        let result = average_of_levels(root);
        println!("res: {:?}", result);
    }
}
