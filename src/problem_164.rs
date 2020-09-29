pub fn maximum_gap(nums: Vec<i32>) -> i32 {
    use std::cmp::*;
    let n = nums.len();
    if n < 2 {
        return 0
    } else if n == 2 {
        return (nums[1] - nums[0]).abs()
    }
    let min_v = nums.iter().fold(nums[0], |acc, &x| min(acc, x));
    let max_v = nums.iter().fold(nums[0], |acc, &x| max(acc, x));
    let b = max((max_v - min_v) / (n as i32 - 1), 1);

    let mut buckets = vec![vec![None, None]; ((max_v - min_v) / b) as usize + 1];
    for num in nums {
        let index = ((num - min_v) / b) as usize;
        buckets[index][0] = Some(buckets[index][0].map_or(num, |k| min(k, num)));
        buckets[index][1] = Some(buckets[index][1].map_or(num, |k| max(k, num)));
    }

    let mut last_max = buckets[0][1].unwrap();
    let mut running_max = 0;
    for i in 0..buckets.len() {
        if let Some(current_min) = buckets[i][0] {
            running_max = max(running_max, current_min - last_max);
            last_max = buckets[i][1].unwrap();
        }
    }
    running_max
}