use crate::listnode::{ListNode, list_to_vec, arr_to_list};

pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
    let m = m as usize;
    let n = n as usize;
    if m == n {
        return head
    }

    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut p = &mut dummy;

    for counter in 0..m-1 {
        p = p.next.as_mut().unwrap();
    }
    let tail_of_unchanged = p;
    let mut p0 = tail_of_unchanged.next.take().unwrap();
    let mut p1 = p0.next.take().unwrap();
    for _ in 0..n-m-1 {
        let mut p2 = p1.next.take().unwrap();
        p1.next = Some(p0);
        p0 = p1;
        p1 = p2;
    }
    let head_of_second_unchanged = p1.next.take();
    p1.next = Some(p0);
    p0 = p1;
    tail_of_unchanged.next = Some(p0);
    let mut tail_of_reversed = tail_of_unchanged;
    for _ in 0..=n-m {
        tail_of_reversed = tail_of_reversed.next.as_mut().unwrap();
    }
    tail_of_reversed.next = head_of_second_unchanged;

    dummy.next
}

#[test]
fn test_reverse_between() {
    assert_eq!(list_to_vec(reverse_between(arr_to_list(&[1, 2, 3, 4, 5]), 2, 4)), vec![1, 4, 3, 2, 5]);
    assert_eq!(list_to_vec(reverse_between(arr_to_list(&[1, 2]), 1, 2)), vec![2, 1]);
}