use rand::prelude::*;

use crate::listnode::ListNode;

struct Solution {
    head: Option<Box<ListNode>>,
    rng: ThreadRng,
}


impl Solution {
    /** @param head The linked list's head.
        Note that the head is guaranteed to be not null, so it contains at least one node. */
    fn new(head: Option<Box<ListNode>>) -> Self {
        Self {
            head,
            rng: thread_rng(),
        }
    }

    /** Returns a random node's value. */
    fn get_random(&mut self) -> i32 {
        let mut scope = 1;
        let mut chosen = 0;
        let mut p = self.head.as_ref();

        while let Some(q) = p {
            if self.rng.gen_range(0, scope) < 1 {
                chosen = q.val;
            }
            p = q.next.as_ref();
            scope += 1;
        }
        chosen
    }
}