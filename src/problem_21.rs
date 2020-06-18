use crate::listnode::*;

pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if l1.is_none() {
        return l2
    }
    if l2.is_none() {
        return l1
    }

    let mut dummy = Box::new(ListNode::new(0));
    let mut p1 = l1;
    let mut p2 = l2;
    let mut p = dummy.as_mut();

    loop {
        match &p1 {
            None => {
                p.next = p2.clone();
                break;
            },
            Some(q1) => {
                match &p2 {
                    None => {
                        p.next = p1.clone();
                        break;
                    },
                    Some(q2) => {
                        if q1.val < q2.val {
                            p.next = p1.clone();
                            p = p.next.as_mut().unwrap();
                            p1 = p1.unwrap().next;
                        } else {
                            p.next = p2.clone();
                            p = p.next.as_mut().unwrap();
                            p2 = p2.unwrap().next;
                        }
                    },
                }
            },
        }
    }

    dummy.next
}

#[test]
fn test_merge_two_lists() {
    assert_eq!(list_to_vec(merge_two_lists(arr_to_list(&[1, 2, 4]), arr_to_list(&[1, 3, 4]))), vec![1, 1, 2, 3, 4, 4]);
}