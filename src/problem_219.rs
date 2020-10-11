pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    use std::cmp::*;
    if k <= 0 {
        return false
    }
    let n = nums.len();
    let k = k as usize;
    if n < 10 {
        for i in 0..n {
            for j in i+1..min(i+k+1, n) {
                if i != j && nums[i] == nums[j] {
                    return true
                }
            }
        }
        false
    } else {
        let mut map = std::collections::HashMap::new();
        for i in 0..n {
            if map.get(&nums[i]).unwrap_or(&0) > &0 {
                return true
            }
            *map.entry(nums[i]).or_insert(0) += 1;
            if i >= k {
                map.entry(nums[i-k]).and_modify(|e| *e -= 1);
            }
        }
        false
    }
}