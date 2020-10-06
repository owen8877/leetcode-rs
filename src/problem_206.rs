use crate::listnode::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if let Some(mut head) = head {
        let mut reversed = None;
        let mut tail = head.next.take();
        while let Some(mut t) = tail {
            head.next = reversed;
            reversed = Some(head);
            let tt = t.next.take();
            head = t;
            tail = tt;
        }
        head.next = reversed;
        Some(head)
    } else {
        None
    }
}