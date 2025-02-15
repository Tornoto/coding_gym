use super::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// https://leetcode.com/problems/binary-tree-right-side-view/
/// 这是一道常见的层次遍历题。只需要遍历数的每一层，并取出最右侧的节点即可
pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
        return vec![];
    }

    let root = root.unwrap();
    let mut queue = VecDeque::new();
    // 根节点入队列
    queue.push_back(root);

    let mut res = vec![];

    while !queue.is_empty() {
        let n = queue.len();
        // 将每一层弹出
        for i in 0..n {
            let cur_node = queue.pop_front().unwrap();
            // 由于右树先入队列，因此第一个元素就是每层最右侧的元素
            if i == 0 {
                res.push(cur_node.borrow().val);
            }
            // 先让右树入队列
            let right_opt = cur_node.borrow().right.clone();
            if let Some(right) = right_opt {
                queue.push_back(right);
            }
            // 再让右树入队列
            let left_opt = cur_node.borrow().left.clone();
            if let Some(left) = left_opt {
                queue.push_back(left);
            }
        }
    }

    res
}
