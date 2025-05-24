use super::list::ListNode;

/// https://leetcode.com/problems/remove-nth-node-from-end-of-list/
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode {
        val: -1,
        next: head,
    });

    let mut fast = dummy.clone();

    for _ in 0..n {
        if let Some(next_node) = fast.next {
            fast = next_node;
        } else {
            return dummy.next;
        }
    }

    let mut slow = &mut dummy;
    // let mut xx = dummy.as_mut();
    while let Some(next_node) = fast.next {
        fast = next_node;
        slow = slow.next.as_mut().unwrap();
    }

    let next = slow.next.take();
    slow.next = next.unwrap().next;

    dummy.next
}
