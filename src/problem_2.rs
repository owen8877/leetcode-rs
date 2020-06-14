#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
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

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut ll = ListNode::new(0);
    let mut l = &mut ll;
    let mut l1m = l1;
    let mut l2m = l2;
    let mut carrier = 0;

    loop {
        let (l1m_has_next, d1) = match l1m {
            None => (false, 0),
            Some(l1b) => {
                l1m = l1b.next;
                (true, l1b.val)
            },
        };
        let (l2m_has_next, d2) = match l2m {
            None => (false, 0),
            Some(l2b) => {
                l2m = l2b.next;
                (true, l2b.val)
            },
        };
        if l1m_has_next || l2m_has_next || carrier > 0 {
            let mut sum = d1 + d2 + carrier;
            if sum < 10 {
                carrier = 0;
            } else {
                sum -= 10;
                carrier = 1;
            }
            l.next = Some(Box::from(ListNode::new(sum)));
            l = l.next.as_mut().unwrap();
        } else {
            return Some(Box::from(*ll.next.unwrap()));
        }
    }
}

#[test]
fn test_add_two_numbers() {
    let l1f = ListNode {
        val: 2,
        next: Some(Box::from(ListNode {
            val: 4,
            next: Some(Box::from(ListNode::new(3))),
        })),
    };

    let l2f = ListNode {
        val: 5,
        next: Some(Box::from(ListNode {
            val: 6,
            next: Some(Box::from(ListNode::new(4))),
        })),
    };

    print!("{:#?}", l1f);
    print!("{:#?}", l2f);
    print!("{:#?}", add_two_numbers(Some(Box::from(l1f)), Some(Box::from(l2f))));
}