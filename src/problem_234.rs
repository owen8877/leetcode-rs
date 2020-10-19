use crate::listnode::ListNode;

fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    head.map_or(true, |head| {
        let mut p = &head;
        let mut arr = vec![p.val];
        while let Some(q) = p.next.as_ref() {
            arr.push(q.val);
            p = q;
        }
        for i in 0..arr.len() / 2 {
            if arr[i] != arr[arr.len() - 1 - i] {
                return false;
            }
        }
        true
    })
}