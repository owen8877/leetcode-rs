use crate::listnode::*;

pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if l1.is_none() {
            return l2;
        }
        if l2.is_none() {
            return l1;
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
                }
                Some(q1) => {
                    match &p2 {
                        None => {
                            p.next = p1.clone();
                            break;
                        }
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
                        }
                    }
                }
            }
        }

        dummy.next
    }

    head.map_or(None, |h| {
        if h.next.is_none() {
            return Some(h);
        }
        let mut dummy = ListNode { val: 0, next: Some(h) };

        let mut p = &mut dummy;
        let mut counter = 0;
        while let Some(next) = p.next.as_mut() {
            p = next;
            counter += 1;
        }

        p = &mut dummy;
        for _ in 0..counter / 2 {
            p = p.next.as_mut().unwrap();
        }

        let l2 = sort_list(p.next.take());
        let l1 = sort_list(dummy.next);

        merge_two_lists(l1, l2)
    })
}

pub fn sort_list_vec(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    pub fn arr_to_list(arr: &[i32]) -> Option<Box<ListNode>> {
        if arr.len() == 0 {
            return None;
        }
        let mut pointer = ListNode::new(arr[arr.len() - 1]);
        for i in (0..arr.len() - 1).rev() {
            pointer = ListNode {
                val: arr[i],
                next: Some(Box::new(pointer)),
            };
        }
        Some(Box::new(pointer))
    }

    pub fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = vec![];
        let mut pointer = list;
        while pointer.is_some() {
            let p = pointer.unwrap();
            vec.push(p.val.clone());
            pointer = p.next;
        }
        vec
    }

    let mut v = list_to_vec(head);
    v.sort();
    arr_to_list(&v)
}

#[test]
fn test_sort_list() {
    assert_eq!(list_to_vec(sort_list_vec(arr_to_list(&[1, 5, 2, 6, 3, 4]))), vec![1, 2, 3, 4, 5, 6])
}