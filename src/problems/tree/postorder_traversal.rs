#![allow(unused)]
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use super::binary_tree::TreeNode;

pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    postorder_iteration_deque(&root, &mut result);
    result
}

fn postorder_recursive(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    if let Some(node) = root {
        postorder_recursive(&node.borrow().left.clone(), result);
        postorder_recursive(&node.borrow().right.clone(), result);
        result.push(node.borrow().val);
    }
}

fn postorder_iteration_deque(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    let mut stack_support = VecDeque::new();
    let mut stack_result = VecDeque::new();

    if let Some(node) = root {
        stack_support.push_back(Some(node.clone()));
    }

    while !stack_support.is_empty() {
        if let Some(node) = stack_support.pop_back().unwrap() {
            stack_support.push_back(node.borrow().left.clone());
            stack_support.push_back(node.borrow().right.clone());
            stack_result.push_front(node);
        }
    }

    while !stack_result.is_empty() {
        result.push(stack_result.pop_front().unwrap().borrow().val);
    }
}

#[cfg(test)]
mod test {
    use crate::problems::tree::binary_tree::get_test_case;

    use super::*;

    #[test]
    fn test_preorder_recursive() {
        let root = get_test_case();
        let result = postorder_traversal(root);
        println!("result: {:?}", result);
        println!("stand : {:?}", vec![4, 6, 7, 5, 2, 9, 8, 3, 1]);
    }
}
