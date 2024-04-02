pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (Some(ref node1), Some(ref node2)) => {
            if node1.borrow().val == node2.borrow().val {
                return Self::is_same_tree(
                    node1.borrow().left.clone(),
                    node2.borrow().left.clone(),
                ) && Self::is_same_tree(
                    node1.borrow().right.clone(),
                    node2.borrow().right.clone(),
                );
            } else {
                return false;
            }
        }
        (None, None) => {
            return true;
        }
        _ => return false,
    }
}
