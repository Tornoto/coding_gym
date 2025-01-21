use crate::problems::tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

/// https://leetcode.com/problems/house-robber-iii/description/
/// 这里的关键是如何选择一种遍历二叉树的方式。
///
/// 对于当前节点，我们需要考虑两种情形：抢劫当前节点和不抢劫当前节点
///
/// 那么那种遍历方式能实现这点呢？
///
/// 层次？层次遍历注重同层的关系，难以维系父子节点的关系
///
/// 先序？先访问当前节点，而当前节点又依赖子节点的抢劫结果，因此先序是不行的。
///
/// 后序是理想的选择，后续先访问了子节点，可以获取子节点的抢劫情况，这时候就是不抢劫当前节点的情形。
///
/// 再考虑抢劫当前节点的情形。而考虑抢劫当前节点的情形，又需要考虑不抢劫子节点的情形。
///
/// 因此子节点返回时，需要提供抢劫自身节点和不抢劫自身节点的信息。
pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let res = rob_tree(root);
    return res.0.max(res.1);
}

/// 返回的是抢劫当前节点和不抢劫当前节点的信息
fn rob_tree(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    if root.is_none() {
        return (0, 0);
    }

    let root = root.unwrap();
    let left_res = rob_tree(root.borrow().left.clone());
    let right_res = rob_tree(root.borrow().right.clone());

    // rob 当前节点, 那么不能抢劫子节点
    let val1 = root.borrow().val + left_res.1 + right_res.1;
    // do not rob 当前节点，那么从选择子节点中的大值
    let val2 = left_res.0.max(left_res.1) + right_res.0.max(right_res.1);

    //注意返回顺序 (rob cur , do not rot cur)
    (val1, val2)
}
