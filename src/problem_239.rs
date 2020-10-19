pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::VecDeque;

    let mut cmax = VecDeque::new();
    let mut result = vec![];

    for i in 0..nums.len() {
        while !cmax.is_empty() && nums[*cmax.back().unwrap()] < nums[i] {
            cmax.pop_back();
        }
        if !cmax.is_empty() && *cmax.front().unwrap() as i32 <= i as i32 - k {
            cmax.pop_front();
        }
        cmax.push_back(i);
        if i as i32 + 1 >= k {
            result.push(nums[*cmax.front().unwrap()]);
        }
    }

    result
}