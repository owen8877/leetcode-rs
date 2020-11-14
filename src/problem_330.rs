pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
    let mut counter = 0;
    let mut current_max = 0;
    let mut j = 0;
    while current_max < n as i64 {
        if j < nums.len() && nums[j] as i64 <= current_max + 1 {
            current_max += nums[j] as i64;
            j += 1;
        } else {
            counter += 1;
            current_max = current_max * 2 + 1;
        }
    }
    counter as i32
}