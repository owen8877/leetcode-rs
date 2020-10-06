pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0
    } else if n == 1 {
        return if nums[0] >= s { 1 } else { 0 }
    }

    let mut sum = 0;
    let mut left = 0;
    let mut min_len = None;
    for (right, &num) in nums.iter().enumerate() {
        sum += num;
        if sum >= s {
            while sum - nums[left] >= s {
                sum -= nums[left];
                left += 1;
            }
            min_len = Some(std::cmp::min(min_len.unwrap_or(n), right-left+1));
        }
    }
    min_len.unwrap_or(0) as i32
}