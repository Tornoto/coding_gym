use super::list::ListNode;

/// https://leetcode.com/problems/swap-nodes-in-pairs/description/
pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode {
        val: -1,
        next: head,
    });
    let mut pre_node = &mut dummy;

    // 注意使用take剥离所有权和借用
    while let Some(mut cur_node) = pre_node.next.take() {
        if let Some(mut next_node) = cur_node.next.take() {
            cur_node.next = next_node.next.take();
            pre_node.next = Some(next_node);
            pre_node = pre_node.next.as_mut().unwrap().next.as_mut().unwrap();
        } else {
            // 如果是奇数个，由于cur_node被take,如果不将其放回则会导致元素缺少
            pre_node.next = Some(cur_node);
            // 如果不再向前走一步，会导致死循环
            pre_node = pre_node.next.as_mut().unwrap();
        }
    }

    dummy.next
}
