pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
    if k <= 0 {
        return false
    } else if t < 0 {
        return false
    }

    use std::collections::BTreeSet;
    let k = k as usize;
    let t = t as i64;

    let mut set = BTreeSet::new();

    for (i, &num) in nums.iter().enumerate() {
        let num = num as i64;
        if set.range(num-t..=num+t).next().is_some() {
            return true
        }
        set.insert(num);
        if i >= k {
            set.remove(&(nums[i-k] as i64));
        }
    }

    false
}