use crate::listnode::*;

pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    if head.is_none() || head.as_ref().unwrap().next.as_ref().is_none() {
        return head
    }

    let mut dummy = Box::new(ListNode { val: 0, next: head });

    loop {
        let mut changed = false;

        let mut p = &mut dummy;
        while let Some(mut q) = p.next.take() {
            if let Some(mut r) = q.next.take() {
                if q.val >= x && x > r.val {
                    let s = r.next.take();
                    q.next = s;
                    r.next = Some(q);
                    p.next = Some(r);
                    changed = true;
                } else {
                    q.next = Some(r);
                    p.next = Some(q);
                }
                p = p.next.as_mut().unwrap();
            } else {
                p.next = Some(q);
                break
            }
        }

        if !changed { break }
    }

    dummy.next
}

#[test]
fn test_partition() {
    assert_eq!(list_to_vec(partition(arr_to_list(&[1, 4, 3, 2, 5, 2]), 3)), vec![1, 2, 2, 4, 3, 5]);
}