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

/// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/description/
/// ugly though, works
pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut values = Vec::new();
    inorder_traversal(root.clone(), &mut values);

    let mut map: HashMap<i32, usize> = HashMap::new();
    for (index, value) in values.iter().enumerate() {
        map.insert(*value, index);
    }

    let vp = p.unwrap().borrow().val;
    let vq = q.unwrap().borrow().val;

    let mut cur = root.clone();
    let mut prev = None;

    while let Some(node) = cur.clone() {
        let vpp = map.get(&vp).unwrap();
        let vqp = map.get(&vq).unwrap();
        let rp = map.get(&node.borrow().val).unwrap();

        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();
        if vpp > rp && vqp > rp {
            prev = cur.clone();
            cur = right;
        } else if vpp < rp && vqp < rp {
            prev = cur.clone();
            cur = left;
        } else {
            return cur;
        }
    }

    prev
}

/// https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-search-tree/description/
pub fn lowest_common_ancestor_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut cur = root;

    let p_val = p.unwrap().borrow().val;
    let q_val = q.unwrap().borrow().val;
    while let Some(node) = cur.clone() {
        let val = node.borrow().val;
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();
        if p_val > val && q_val > val {
            cur = right;
        } else if p_val < val && q_val < val {
            cur = left;
        } else {
            return Some(node);
        }
    }
    None
}

///https://leetcode.com/problems/delete-node-in-a-bst/description/
pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root.clone() {
        if key > node.borrow().val {
            let right = node.borrow().right.clone();
            node.borrow_mut().right = delete_node(right, key);
        } else if key < node.borrow().val {
            let left = node.borrow().left.clone();
            node.borrow_mut().left = delete_node(left, key);
        } else {
            // find the target node
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            // if left tree is None, after deleting root, only right tree left
            if left.is_none() {
                node.borrow_mut().right = None;
                return right;
            }
            // if right tree is None, after deleting root, only left tree left
            if right.is_none() {
                node.borrow_mut().left = None;
                return left;
            }
            // if both left and rgith trees exist
            // find the smallest node in right tree
            // copy its val to root
            // then delete the smallest node in the right tree
            if let Some(min_node) = find_min(right.clone()) {
                let new_key = min_node.borrow().val;
                node.borrow_mut().val = new_key;
                node.borrow_mut().right = delete_node(right, new_key);
            }
        }
    }
    root
}

fn find_min(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return None;
    }
    let mut root = root;
    while let Some(node) = root.clone().unwrap().borrow().left.clone() {
        root = Some(node);
    }

    return root;
}
