use rand::prelude::*;

struct Solution {
    nums: Vec<i32>,
    rng: ThreadRng,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            nums,
            rng: thread_rng(),
        }
    }

    /** Resets the array to its original configuration and return it. */
    fn reset(&self) -> Vec<i32> {
        self.nums.clone()
    }

    /** Returns a random shuffling of the array. */
    fn shuffle(&mut self) -> Vec<i32> {
        let mut s = self.nums.clone();
        for i in 0..s.len() {
            let j = self.rng.gen_range(i, s.len());
            s.swap(i, j);
        }
        s
    }
}