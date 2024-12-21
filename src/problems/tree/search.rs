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

/// https://leetcode.com/problems/trim-a-binary-search-tree/description/
pub fn trim_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    low: i32,
    high: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root.clone() {
        let val = node.borrow().val;
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();
        if val < low {
            return trim_bst(right, low, high);
        } else if val > high {
            return trim_bst(left, low, high);
        } else {
            let mut node_mut = node.borrow_mut();
            node_mut.left = trim_bst(left, low, high);
            node_mut.right = trim_bst(right, low, high);
        }
    }
    root
}

/// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/description/
///
pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    sorted_array_to_bst_sub(&nums)
}

pub fn sorted_array_to_bst_sub(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }

    if nums.len() == 1 {
        return Some(Rc::new(RefCell::new(TreeNode::new(nums[0]))));
    }

    let mid = (nums.len() - 1) / 2;

    let root = Rc::new(RefCell::new(TreeNode::new(nums[mid])));

    root.borrow_mut().left = sorted_array_to_bst_sub(&nums[..mid]);
    root.borrow_mut().right = sorted_array_to_bst_sub(&nums[mid + 1..]);

    Some(root)
}

/// ```plaintext
/// CONVERT BST
///      10
///     /  \
///    5    15
///   / \   / \
///  3   7 12  18
/// TO
///      70
///     /  \
///   15    45
///   / \   / \
///  3   7 12  18
///
/// ```
pub fn convert_bst_to_accumulated_sum(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    convert_bst_sub(root.clone());
    root
}

pub fn convert_bst_sub(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(root) = root {
        let left_opt = root.borrow().left.clone();
        let right_opt = root.borrow().right.clone();

        root.borrow_mut().val += convert_bst_sub(left_opt);
        root.borrow_mut().val += convert_bst_sub(right_opt);

        root.borrow().val
    } else {
        0
    }
}

/// https://leetcode.com/problems/convert-bst-to-greater-tree/description/
/// ```plaintext
/// CONVERT BST
///        4
///     /     \
///    1       6
///   / \     / \
///  0   2   5   7
///       \       \
///        3       8
///
/// TO
///        30
///     /      \
///    36      21
///   / \     /  \
///  36 35   26   15
///       \        \
///       33        8
/// ```
/// 如下有两个版本的递归
/// 一个是递归修改 cumulative_sum
/// 一个是向左树传递之前的累计和，相较而言第一种更简洁
pub fn convert_to_greater_sum_tree(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut cumulative_sum = 0;
    reverse_inorder(&root, &mut cumulative_sum);
    // convert_to_gst_sub(root.clone(), 0);
    root
}

/// 由于是右节点优先的中序遍历，遍历的过程就是节点正确累加的过程
fn reverse_inorder(root: &Option<Rc<RefCell<TreeNode>>>, cumulative_sum: &mut i32) {
    if let Some(node) = root {
        let mut node_mut = node.borrow_mut();
        // Traverse the right subtree first
        reverse_inorder(&node_mut.right, cumulative_sum);
        // Update the node's value
        *cumulative_sum += node_mut.val;
        node_mut.val = *cumulative_sum;
        // Traverse the left subtree
        reverse_inorder(&node_mut.left, cumulative_sum);
    }
}

/// 这个也是右节点优先的中序遍历
/// 向右树遍历时，将parent_val 传给右树；右树遍历完毕后，更新root的累加值
/// 在向左侧遍历，将parent_val 更新为 root 当前的累加值，传到左树
/// 如果节点是空，则返回parent_val
/// 如果节点没有子节点，则更新自己的累加值 然后返回
/// 对比下来，这是一种不简洁的实现方式
/// the simpler, the better
pub fn convert_to_gst_sub(root: Option<Rc<RefCell<TreeNode>>>, parent_val: i32) -> i32 {
    if let Some(root) = root {
        let mut val = root.borrow().val;
        let left = root.borrow().left.clone();
        let right = root.borrow().right.clone();
        if left.is_none() && right.is_none() {
            val += parent_val;
            root.borrow_mut().val = val;
            return val;
        }
        val += convert_to_gst_sub(right, parent_val);
        root.borrow_mut().val = val;
        convert_to_gst_sub(left, val)
    } else {
        parent_val
    }
}
