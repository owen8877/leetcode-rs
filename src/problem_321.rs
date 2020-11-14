pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
    use std::cmp::*;
    use std::cmp::Ordering::*;

    let k = k as usize;
    let mut vm = vec![];

    fn find_max(v: &[i32], k: usize) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut to_pop = v.len() - k;
        for &n in v {
            while to_pop > 0 && !result.is_empty() && result.last().unwrap() < &n {
                result.pop();
                to_pop -= 1;
            }
            result.push(n);
        }
        result.resize(k, 0);
        result
    }

    fn merge(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        let mut i1 = 0;
        let mut i2 = 0;
        while i1 < v1.len() && i2 < v2.len() {
            if &v1[i1..] > &v2[i2..] {
                result.push(v1[i1]);
                i1 += 1;
            } else {
                result.push(v2[i2]);
                i2 += 1;
            }
        }
        if i1 < v1.len() {
            for j in i1..v1.len() {
                result.push(v1[j]);
            }
        }
        if i2 < v2.len() {
            for j in i2..v2.len() {
                result.push(v2[j]);
            }
        }
        result
    }

    for i in 0..=min(nums1.len(), k) {
        if k - i > nums2.len() {
            continue;
        }
        let v1 = find_max(&nums1, i);
        let v2 = find_max(&nums2, k - i);
        vm = max(vm, merge(v1, v2));
    }
    vm
}