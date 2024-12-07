#![allow(unused)]

use super::binary_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];
    inorder_recursive(root, &mut result);
    result
}

fn inorder_recursive(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    if let Some(node) = root {
        inorder_recursive(node.borrow().left.clone(), result);
        result.push(node.borrow().val);
        inorder_recursive(node.borrow().right.clone(), result);
    }
}

fn inorder_iteration(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];

    let mut cur_node = root;

    while !cur_node.is_none() || !stack.is_empty() {
        while let Some(node) = cur_node {
            stack.push(node.clone());
            cur_node = node.borrow().left.clone();
        }

        if let Some(node) = stack.pop() {
            result.push(node.borrow().val);
            cur_node = node.borrow().right.clone();
        }
    }
}

pub fn inorder_traversal_iteration(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut stack = Vec::new();
    let mut cur_node = root;

    while !cur_node.is_none() || !stack.is_empty() {
        while let Some(node) = cur_node {
            stack.push(node.clone());
            cur_node = node.borrow().left.clone();
        }

        if let Some(node) = stack.pop() {
            result.push(node.borrow().val);
            cur_node = node.borrow().right.clone();
        }
    }

    result
}

/// Morris algorithm to solve inorder traversal
/// [Inorder Tree Traversal without recursion and without stack!](https://www.geeksforgeeks.org/inorder-tree-traversal-without-recursion-and-without-stack/)
pub fn inorder_morris(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];
    let mut cur = root;

    while let Some(node) = cur.clone() {
        // if cur node has a left child
        if let Some(left) = node.borrow().left.clone() {
            // get the predecessor of cur node
            let mut prev = left;
            while let Some(right) = prev.clone().borrow().right.clone() {
                // if right.as_ptr() == node.as_ptr() {
                //     break;
                // }
                if *right.borrow() == *node.borrow() {
                    break;
                }
                prev = right;
            }

            // if predecessor has right child, in this condition, the child is some 'cur' node
            //Revert the changes made in the tree structure
            if prev.borrow().right.is_some() {
                result.push(node.borrow().val);
                prev.borrow_mut().right = None;
                cur = node.borrow().right.clone();
            } else {
                // if predecessor of cur node has no right child
                // set cur node as it's predecessor's right child
                prev.borrow_mut().right = Some(node.clone());
                // move cur node to its left child
                cur = node.borrow().left.clone();
            }
        } else {
            result.push(node.borrow().val);
            cur = node.borrow().right.clone();
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::problems::tree::binary_tree;

    #[test]
    fn test_preorder_recursive() {
        let root = binary_tree::get_test_case();
        let result = inorder_traversal(root.clone());
        println!("result: {:?}", result);
        println!("stand : {:?}", vec![4, 2, 6, 5, 7, 1, 3, 9, 8]);
        let result = inorder_morris(root.clone());
        println!("result: {:?}", result);
    }
}
