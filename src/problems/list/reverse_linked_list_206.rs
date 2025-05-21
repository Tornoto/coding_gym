use super::list::ListNode;

/// https://leetcode.com/problems/reverse-linked-list/description/
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut cur = head;
    while let Some(mut node) = cur {
        cur = node.next;
        node.next = prev;
        prev = Some(node);
    }
    prev
}
