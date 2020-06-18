#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

pub fn arr_to_list(arr: &[i32]) -> Option<Box<ListNode>> {
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