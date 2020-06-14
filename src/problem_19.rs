#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

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

fn arr_to_list(arr: &[i32]) -> Option<Box<ListNode>> {
    if arr.len() == 0 {
        return None
    }
    let mut pointer = ListNode::new(arr[arr.len()-1]);
    for i in (0..arr.len()-1).rev() {
        pointer = ListNode {
            val: arr[i],
            next: Some(Box::new(pointer))
        };
    }
    Some(Box::new(pointer))
}

fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = vec![];
    let mut pointer = list;
    while pointer.is_some() {
        let p = pointer.unwrap();
        vec.push(p.val.clone());
        pointer = p.next;
    }
    vec
}

#[test]
fn test_remove_nth_from_end() {
    let list = arr_to_list(&[1, 2, 3, 4, 5]);
    assert_eq!(list_to_vec(remove_nth_from_end(list, 2)), vec![1, 2, 3, 5]);
}