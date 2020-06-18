use crate::listnode::*;

pub fn merge_k_lists_v0(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    if lists.len() == 0 {
        return None
    }
    if lists.len() == 1 {
        return lists[0].clone()
    }

    let mut heads = vec![];
    let mut val_pos = vec![];
    for i in 0..lists.len() {
        heads.push(lists[i].as_ref());
        match &lists[i] {
            None => {},
            Some(head) => {
                val_pos.push((head.val, i));
            },
        }
    }
    val_pos.sort_by(|&a, b| {
        a.0.cmp(&b.0)
    });

    let mut dummy = Box::new(ListNode::new(0));
    let mut p = dummy.as_mut();

    while val_pos.len() > 0 {
        let (val, index) = val_pos.remove(0);
        let current_node = heads[index].unwrap();
        match current_node.next.as_ref() {
            None => {},
            Some(next_node) => {
                val_pos.push((next_node.val, index));
                heads[index] = Some(next_node);
                val_pos.sort_by(|&a, b| {
                    a.0.cmp(&b.0)
                });
            },
        }
        p.next = Some(Box::new(ListNode::new(val)));
        p = p.next.as_mut().unwrap();
    }

    dummy.next
}

// Slower, don't know why
pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    use std::cmp::Ordering;
    if lists.len() == 0 {
        return None
    }
    if lists.len() == 1 {
        return lists[0].clone()
    }

    let mut heads = vec![];
    let mut val_pos = vec![];
    for i in 0..lists.len() {
        heads.push(lists[i].as_ref());
        match &lists[i] {
            None => {
                val_pos.push(None);
            },
            Some(head) => {
                val_pos.push(Some((head.val, i)));
            },
        }
    }
    val_pos.sort_by(|a, b| {
        match a {
            None => match b {
                None => Ordering::Equal,
                Some(_) => Ordering::Greater,
            },
            Some(aa) => match b {
                None => Ordering::Less,
                Some(bb) => aa.0.cmp(&bb.0),
            },
        }
    });

    let mut dummy = Box::new(ListNode::new(0));
    let mut p = dummy.as_mut();

    while val_pos[0].is_some() {
        let (val, index) = val_pos[0].unwrap();
        let current_node = heads[index].unwrap();
        match current_node.next.as_ref() {
            None => {
                val_pos[0] = None;
            },
            Some(next_node) => {
                val_pos[0] = (Some((next_node.val, index)));
                heads[index] = Some(next_node);
            },
        }
        val_pos.sort_by(|a, b| {
            match a {
                None => match b {
                    None => Ordering::Equal,
                    Some(_) => Ordering::Greater,
                },
                Some(aa) => match b {
                    None => Ordering::Less,
                    Some(bb) => aa.0.cmp(&bb.0),
                },
            }
        });
        p.next = Some(Box::new(ListNode::new(val)));
        p = p.next.as_mut().unwrap();
    }

    dummy.next
}

#[test]
fn test_merge_k_lists() {
    assert_eq!(merge_k_lists(vec![
        arr_to_list(&[1, 4, 5]),
        arr_to_list(&[1, 3, 4]),
        arr_to_list(&[2, 6]),
    ]), arr_to_list(&[1, 1, 2, 3, 4, 4, 5, 6]));
    assert_eq!(merge_k_lists(vec![
        arr_to_list(&[1, 4, 5]),
    ]), arr_to_list(&[1, 4, 5]));
    assert_eq!(merge_k_lists(vec![
        arr_to_list(&[]),
        arr_to_list(&[]),
    ]), arr_to_list(&[]));
}