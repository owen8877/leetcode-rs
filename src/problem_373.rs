use std::cmp::Ordering;

#[derive(Eq)]
struct Pair {
    i: usize,
    j: usize,
    a: i32,
    b: i32,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.a + other.b).cmp(&(self.a + self.b))
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.a + self.b == other.a + other.b
    }
}

pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let n1 = nums1.len();
    let n2 = nums2.len();
    if (n1 * n2) as i32 <= k {
        let mut answer = vec![];

        for &num1 in &nums1 {
            for &num2 in &nums2 {
                answer.push(vec![num1, num2]);
            }
        }

        answer
    } else {
        let mut heap = std::collections::BinaryHeap::new();
        let mut answer = vec![];

        for i in 0..n1 {
            heap.push(Pair { i, j: 0, a: nums1[i], b: nums2[0] });
        }

        while let Some(p) = heap.pop() {
            let i = p.i;
            let j = p.j;
            answer.push(vec![p.a, p.b]);
            if answer.len() == k as usize {
                break;
            }
            if j + 1 < n2 {
                heap.push(Pair { i: i, j: j + 1, a: nums1[i], b: nums2[j + 1] });
            }
        }

        answer
    }
}