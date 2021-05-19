use std::collections::*;

use rand::prelude::*;

struct RandomizedCollection {
    map: HashMap<i32, usize>,
    rng: ThreadRng,
    sum: usize,
}

impl RandomizedCollection {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            rng: thread_rng(),
            sum: 0,
        }
    }

    /** Inserts a value to the collection. Returns true if the collection did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        self.sum += 1;
        if let Some(v) = self.map.get_mut(&val) {
            *v += 1;
            false
        } else {
            self.map.insert(val, 1);
            true
        }
    }

    /** Removes a value from the collection. Returns true if the collection contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        if let Some(v) = self.map.get_mut(&val) {
            *v -= 1;
            self.sum -= 1;
            if *v == 0 {
                self.map.remove(&val);
            }
            true
        } else {
            false
        }
    }

    /** Get a random element from the collection. */
    fn get_random(&mut self) -> i32 {
        let mut remaining = self.rng.gen_range(0, self.sum);
        for (k, v) in self.map.iter() {
            if remaining < *v {
                return *k;
            }
            remaining -= v;
        }
        0
    }
}