use super::binary_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    preorder_iteration(&root, &mut result);
    result
}

pub fn preorder_recursive(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    if let Some(root_node) = root {
        result.push(root_node.borrow_mut().val);
        preorder_recursive(&root_node.borrow().left, result);
        preorder_recursive(&root_node.borrow().right, result);
    }
}

pub fn preorder_iteration(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    let mut stack = Vec::new();
    if let Some(node) = root {
        stack.push(node.clone());
    }

    while let Some(node) = stack.pop() {
        result.push(node.borrow().val);

        if let Some(right) = &node.borrow().right {
            stack.push(right.clone());
        }

        if let Some(left) = &node.borrow().left {
            stack.push(left.clone())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::problems::tree::binary_tree;

    #[test]
    fn test_preorder_recursive() {
        let root = binary_tree::get_test_case();
        let result = preorder_traversal(root);
        println!("result: {:?}", result);
        println!("stand : {:?}", vec![1, 2, 4, 5, 6, 7, 3, 8, 9]);
    }
}
