use super::binary_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

/// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/description/
pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if inorder.is_empty() || postorder.is_empty() {
        return None;
    }

    build_tree_rec(&inorder, &postorder)
}

pub fn build_tree_rec(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if inorder.is_empty() || postorder.is_empty() {
        return None;
    }
    let root_val = postorder.last().unwrap();
    let root = Rc::new(RefCell::new(TreeNode::new(*root_val)));

    if let Some(pos_of_root_in_inorder) = inorder.iter().position(|&x| x == *root_val) {
        root.borrow_mut().left = build_tree_rec(
            &inorder[0..pos_of_root_in_inorder],
            &postorder[0..pos_of_root_in_inorder],
        );
        root.borrow_mut().right = build_tree_rec(
            &inorder[pos_of_root_in_inorder + 1..],
            &postorder[pos_of_root_in_inorder..postorder.len() - 1],
        )
    }

    Some(root)
}

/// https://leetcode.com/problems/maximum-binary-tree/description/
pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    cmbt_rec(&nums)
}

pub fn cmbt_rec(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(0)));
    if let Some((idx, &max)) = nums.iter().enumerate().max_by_key(|(_, &val)| val) {
        root.borrow_mut().val = max;
        root.borrow_mut().left = cmbt_rec(&nums[0..idx]);
        root.borrow_mut().right = cmbt_rec(&nums[idx + 1..])
    }

    Some(root)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rebuild() {
        let inorder = vec![9, 3, 15, 20, 7];
        let postorder = vec![9, 15, 7, 20, 3];
        let result = build_tree(inorder, postorder);
        println!("{:?}", result);

        let inorder = vec![-1];
        let postorder = vec![-1];
        let result = build_tree(inorder, postorder);
        println!("{:?}", result);

        let inorder = vec![11, 9, 5, 12, 4, 3, 15, 20, 7];
        let postorder = vec![11, 5, 4, 12, 9, 15, 7, 20, 3];
        let result = build_tree(inorder, postorder);
        println!("{:?}", result);
    }

    #[test]
    fn test_cmbt() {
        let nums = vec![3, 2, 1, 6, 0, 5];
        let result = construct_maximum_binary_tree(nums);
        println!("{:?}", result);

        let nums = vec![3];
        let result = construct_maximum_binary_tree(nums);
        println!("{:?}", result);
    }
}
