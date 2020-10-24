pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    if nums.len() <= 1 {
        return nums.len() as i32;
    }
    use std::cmp::*;
    let n = nums.len();
    let mut dp = vec![1; n];
    let mut largest = 0;

    for i in (0..n - 1).rev() {
        for j in i + 1..n {
            if nums[i] >= nums[j] {
                continue;
            }
            dp[i] = max(dp[i], dp[j] + 1);
        }
        largest = max(largest, dp[i]);
    }
    largest
}