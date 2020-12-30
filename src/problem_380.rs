use std::collections::*;

use rand::prelude::*;

struct RandomizedSet {
    set: HashSet<i32>,
    rng: ThreadRng,
}

impl RandomizedSet {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            set: HashSet::new(),
            rng: thread_rng(),
        }
    }

    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        if self.set.contains(&val) {
            false
        } else {
            self.set.insert(val);
            true
        }
    }

    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        if self.set.contains(&val) {
            self.set.remove(&val);
            true
        } else {
            false
        }
    }

    /** Get a random element from the set. */
    fn get_random(&mut self) -> i32 {
        let n = self.set.len();
        let i = self.rng.gen_range(0, n);
        let mut itr = self.set.iter();
        for _ in 0..i {
            itr.next();
        }
        *itr.next().unwrap()
    }
}