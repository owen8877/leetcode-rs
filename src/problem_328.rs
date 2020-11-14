use crate::listnode::ListNode;

pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut v = vec![];
    let mut p = &head;
    while let Some(q) = p.as_ref() {
        v.push(q.val);
        p = &q.next;
    }

    let mut dummy = Box::new(ListNode::new(0));
    let mut p = &mut dummy;

    let mut i = 0;
    while i < v.len() {
        p.next = Some(Box::new(ListNode::new(v[i])));
        p = p.next.as_mut().unwrap();
        i += 2;
    }

    i = 1;
    while i < v.len() {
        p.next = Some(Box::new(ListNode::new(v[i])));
        p = p.next.as_mut().unwrap();
        i += 2;
    }

    dummy.next
}