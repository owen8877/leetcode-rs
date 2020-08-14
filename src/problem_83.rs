pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode {
        val: 0,
        next: head,
    });

    fn recursive_remove(head: &mut Box<ListNode>) {
        if let Some(mut reference) = head.next.take() {
            while let Some(next) = reference.next.take() {
                if next.val == reference.val {
                    reference = next;
                } else {
                    reference.next = Some(next);
                    break;
                }
            }

            recursive_remove(&mut reference);
            head.next = Some(reference);
        }
    }

    recursive_remove(&mut dummy);
    dummy.next
}