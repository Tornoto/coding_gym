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

    #[test]
    fn test_preorder_recursive() {
        let tn1: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(1)));
        let tn2: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(2)));
        let tn3: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(3)));
        let tn4: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(4)));
        let tn5: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(5)));
        let tn6: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(6)));
        let tn7: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(7)));
        let tn8: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(8)));
        let tn9: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(9)));
        tn1.borrow_mut().set(Some(tn2.clone()), Some(tn3.clone()));
        tn2.borrow_mut().set(Some(tn4), Some(tn5.clone()));
        tn3.borrow_mut().set(None, Some(tn8.clone()));
        tn5.borrow_mut().set(Some(tn6), Some(tn7));
        tn8.borrow_mut().set(Some(tn9), None);

        let result = preorder_traversal(Some(tn1));

        println!("result: {:?}", result);
        println!("stand : {:?}", vec![1, 2, 4, 5, 6, 7, 3, 8, 9]);
    }
}
