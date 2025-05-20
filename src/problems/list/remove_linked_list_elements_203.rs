use super::list::ListNode;

pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut cur = &mut dummy;

    while let Some(next_node) = &mut cur.next {
        if next_node.val == val {
            cur.next = next_node.next.take();
        } else {
            cur = cur.next.as_mut().unwrap();
        }
    }

    dummy.next
}
