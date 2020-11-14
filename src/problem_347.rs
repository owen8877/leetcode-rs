use std::cmp::Ordering;

#[derive(Eq, Debug)]
struct Pair {
    num: i32,
    count: usize,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::*;

        let mut map = HashMap::new();
        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }
        let mut heap = BinaryHeap::new();
        for (&num, &count) in map.iter() {
            heap.push(Pair { num, count });
        }
        let mut result = vec![];
        for _ in 0..k as usize {
            result.push(heap.pop().unwrap().num);
        }
        result
    }
}