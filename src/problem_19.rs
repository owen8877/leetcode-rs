// Reference: https://leetcode.com/problems/remove-nth-node-from-end-of-list/discuss/414563/rust-safe-code-one-pass(has-to-use-clone-to-bypass-borrow-checker)
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    dummy.next = head;
    let mut dummy = Box::new(dummy);
    let mut first = dummy.clone();
    let mut second = dummy.as_mut();

    for _ in 0..n {
        first = first.next.unwrap();
    }

    while first.next.is_some() {
        first = first.next.unwrap();
        second = second.next.as_mut().unwrap();
    }

    let next = second.next.as_ref().unwrap(); // The only difference
    second.next = next.next.clone();

    dummy.next
}

#[test]
fn test_remove_nth_from_end() {
    let list = arr_to_list(&[1, 2, 3, 4, 5]);
    assert_eq!(list_to_vec(remove_nth_from_end(list, 2)), vec![1, 2, 3, 5]);
}