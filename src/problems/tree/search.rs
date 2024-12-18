use std::{cell::RefCell, collections::HashMap, i32, i64, rc::Rc};

use super::TreeNode;

/// https://leetcode.com/problems/search-in-a-binary-search-tree/
pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let mut cur = root;

    while let Some(node) = cur.clone() {
        if node.borrow().val == val {
            return cur;
        }
        if node.borrow().val < val {
            cur = node.borrow().right.clone();
        } else {
            cur = node.borrow().left.clone();
        }
    }

    None
}

/// https://leetcode.com/problems/validate-binary-search-tree/description/
pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    bst_validator(root, i32::MIN as i64 - 1, i32::MAX as i64 + 1)
}

fn bst_validator(root: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
    if let Some(node) = root {
        let val = node.borrow().val as i64;
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();

        if val < max && val > min {
            return bst_validator(left, min, val) && bst_validator(right, val, max);
        } else {
            return false;
        }
    }

    true
}

/// https://leetcode.com/problems/minimum-absolute-difference-in-bst/description/
pub fn get_minimum_difference_rec(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut values = Vec::new();
    inorder_traversal(root, &mut values);

    let mut min_diff = i32::MAX;
    for i in 1..values.len() {
        min_diff = min_diff.min((values[i] - values[i - 1]).abs());
    }

    min_diff
}

fn inorder_traversal(node: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
    if let Some(n) = node {
        inorder_traversal(n.borrow().left.clone(), values);
        values.push(n.borrow().val);
        inorder_traversal(n.borrow().right.clone(), values);
    }
}

/// inorder traversal without recursion
/// no need to store poped nodes in vec, just record prev poped node
/// and upate diff when new node poped out
pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut cur = root;
    let mut stack = vec![];
    let mut prev = 0;
    let mut min_diff = i32::MAX;

    while cur.is_some() || !stack.is_empty() {
        while let Some(node) = cur.clone() {
            let left = node.borrow().left.clone();
            stack.push(node);
            cur = left;
        }

        // now cur must be None
        // pop a node from stack
        cur = stack.pop();
        if let Some(node) = cur.clone() {
            let diff = i32::abs(node.borrow().val - prev);
            if diff == 1 {
                return diff;
            }
            if diff < min_diff {
                min_diff = diff;
            }
            prev = node.borrow().val;
            // point to right child
            cur = node.borrow().right.clone();
        }
    }

    min_diff
}

/// https://leetcode.com/problems/find-mode-in-binary-search-tree/
pub fn find_mode_force(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut stack = vec![];
    let mut result = vec![];

    if let Some(node) = root {
        stack.push(node);
    }

    while !stack.is_empty() {
        if let Some(node) = stack.pop() {
            let val = node.borrow().val;
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            map.entry(val).and_modify(|e| *e += 1).or_insert(1);
            if let Some(node) = left {
                stack.push(node);
            }
            if let Some(node) = right {
                stack.push(node);
            }
        }
    }

    let mut max = i32::MIN;
    for (_, val) in map.iter() {
        if *val > max {
            max = *val;
        }
    }
    for (key, val) in map.iter() {
        if *val == max {
            result.push(*key);
        }
    }

    result
}
