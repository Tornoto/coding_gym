use super::binary_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];
    inorder_recursive(&root, &mut result);
    result
}

fn inorder_recursive(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    if let Some(node) = root {
        inorder_recursive(&node.borrow().left, result);
        result.push(node.borrow().val);
        inorder_recursive(&node.borrow().right, result);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::problems::tree::binary_tree;

    #[test]
    fn test_preorder_recursive() {
        let root = binary_tree::get_test_case();
        let result = inorder_traversal(root);
        println!("result: {:?}", result);
        println!("stand : {:?}", vec![4, 2, 6, 5, 7, 1, 3, 9, 8]);
    }
}
