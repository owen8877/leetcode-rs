pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut p = &mut dummy;
    while let Some(q) = p.next.take() {
        if q.val == val {
            p.next = q.next;
        } else {
            p.next = Some(q);
            p = p.next.as_mut().unwrap();
        }
    }
    dummy.next
}