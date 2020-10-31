use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

#[derive(Eq, Debug)]
struct Pair {
    val: i32,
    index: usize,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::new();
    let mut arr = vec![1];
    let mut pointers = vec![0; primes.len()];

    for (i, &prime) in primes.iter().enumerate() {
        heap.push(Reverse(Pair { val: prime, index: i }));
    }

    let mut i = 0;
    while i < n as usize {
        if let Reverse(next) = heap.pop().unwrap() {
            if arr[i] != next.val {
                arr.push(next.val);
                i += 1;
            }
            pointers[next.index] += 1;
            heap.push(Reverse(Pair { val: arr[pointers[next.index]] * primes[next.index], index: next.index }));
        }
    }
    arr[n as usize - 1]
}