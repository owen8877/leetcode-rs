use crate::listnode::*;
use std::borrow::BorrowMut;

// Warning: not working, needs review
pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode {val: 0, next: head});

    fn reverse(mut start: &Box<ListNode>, k: i32) {

    }

    let mut p = dummy.as_mut();
    'outer: loop {
        let mut q = p.borrow_mut();
        for _ in 0..k {
            match &mut q.next {
                None => break 'outer,
                Some(q1) => {
                    q = q1;
                },
            }
        }

        let mut tail = p.next.as_mut().unwrap().clone();
        let mut remaining = tail.next.as_mut().unwrap();
        for _ in 0..k-1 {
            let start_next = p.next.as_mut().unwrap();
            let mut second_remaining = remaining.next.as_mut().unwrap();
            remaining.next = Some(start_next.clone());
            p.next = Some(remaining.to_owned());
            remaining = second_remaining;
        }
        tail.next = Some(remaining.clone());
    }

    dummy.next
}

#[test]
fn test_swap_pairs() {
    assert_eq!(reverse_k_group(arr_to_list(&[2, 1, 4, 3, 6, 5]), 2), arr_to_list(&[1, 2, 3, 4, 5, 6]));
    assert_eq!(reverse_k_group(arr_to_list(&[2, 1, 4, 3, 5]), 3), arr_to_list(&[4, 1, 2, 3, 5]));
    assert_eq!(reverse_k_group(arr_to_list(&[2, 1, 4, 3]), 3), arr_to_list(&[4, 1, 2, 3]));
    assert_eq!(reverse_k_group(arr_to_list(&[2, 1, 4]), 4), arr_to_list(&[2, 1, 4]));
    assert_eq!(reverse_k_group(arr_to_list(&[2, 1]), 2), arr_to_list(&[1, 2]));
    assert_eq!(reverse_k_group(arr_to_list(&[1]), 2), arr_to_list(&[1]));
    assert_eq!(reverse_k_group(arr_to_list(&[]), 7), arr_to_list(&[]));
}