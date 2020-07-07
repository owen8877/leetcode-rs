use crate::listnode::*;

// Reference: https://leetcode.com/problems/rotate-list/discuss/501528/Rust-with-Explain
pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    fn len(head: Option<&Box<ListNode>>, h: usize) -> usize {
        match head {
            None => h,
            Some(n) => len(n.next.as_ref(), h+1),
        }
    }

    let length = len(head.as_ref(), 0);
    if length == 0 {
        return None
    }
    let k = (k as usize) % length;
    if k == 0 {
        return head
    }

    let mut ahead = head.as_mut().unwrap();
    for _ in 0..length-k-1 {
        ahead = ahead.next.as_mut().unwrap();
    }

    let mut new_head = ahead.next.take();
    let mut ptr = new_head.as_mut();
    while let Some(node) = ptr {
        if node.next.is_none() {
            ptr = Some(node);
            break
        }
        ptr = node.next.as_mut();
    }
    ptr.unwrap().next = head;

    new_head
}

#[test]
fn test_rotate_right() {
    assert_eq!(list_to_vec(rotate_right(arr_to_list(&[1, 2, 3, 4, 5]), 2)), vec![4, 5, 1, 2, 3]);
    assert_eq!(list_to_vec(rotate_right(arr_to_list(&[0, 1, 2]), 4)), vec![2, 0, 1]);
}