pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    for &num in nums.iter() {
        set.insert(num);
    }

    let mut longest = 0;
    for num in nums {
        if !set.contains(&(num-1)) {
            let mut current = num;
            while set.contains(&(current+1)) {
                current += 1;
            }
            longest = std::cmp::max(longest, current-num+1);
        }
    }
    longest
}