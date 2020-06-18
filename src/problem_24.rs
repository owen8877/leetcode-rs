use crate::listnode::*;

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    let mut p = dummy.as_mut();

    fn core(p: &mut ListNode) {
        match p.next.as_ref() {
            None => {},
            Some(n1) => {
                match n1.next.as_ref() {
                    None => {},
                    Some(n2) => {
                        let mut m2 = ListNode::new(n1.val);
                        let mut m1 = ListNode::new(n2.val);
                        m2.next = n2.next.as_ref().cloned();
                        core(&mut m2);
                        m1.next = Some(Box::new(m2));
                        p.next = Some(Box::new(m1));
                    },
                }
            }
        }
    }

    core(p);
    dummy.next
}

#[test]
fn test_swap_pairs() {
    assert_eq!(swap_pairs(arr_to_list(&[2, 1, 4, 3, 6, 5])), arr_to_list(&[1, 2, 3, 4, 5, 6]));
    assert_eq!(swap_pairs(arr_to_list(&[2, 1, 4, 3, 5])), arr_to_list(&[1, 2, 3, 4, 5]));
    assert_eq!(swap_pairs(arr_to_list(&[2, 1, 4, 3])), arr_to_list(&[1, 2, 3, 4]));
    assert_eq!(swap_pairs(arr_to_list(&[2, 1, 4])), arr_to_list(&[1, 2, 4]));
    assert_eq!(swap_pairs(arr_to_list(&[2, 1])), arr_to_list(&[1, 2]));
    assert_eq!(swap_pairs(arr_to_list(&[1])), arr_to_list(&[1]));
    assert_eq!(swap_pairs(arr_to_list(&[])), arr_to_list(&[]));
}