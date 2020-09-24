use crate::listnode::ListNode;

pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut sorted = Box::new(ListNode {
        val: (-1) << 31,
        next: None,
    });
    while let Some(mut node) = head {
        let mut p = &mut sorted;
        while let Some(q) = p.next.as_ref() {
            if q.val >= node.val && node.val >= p.val {
                break
            }
            p = p.next.as_mut().unwrap();
        }
        head = node.next.take();
        node.next = p.next.take();
        p.next = Some(node);
    }
    sorted.next
}