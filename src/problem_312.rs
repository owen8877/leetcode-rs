pub fn max_coins(nums: Vec<i32>) -> i32 {
    use std::cmp::*;

    let n = nums.len();
    if n == 0 {
        0
    } else if n == 1 {
        nums[0]
    } else if n == 2 {
        nums[0] * nums[1] + max(nums[0], nums[1])
    } else {
        let mut dp = vec![vec![0; n]; n];

        fn core(i: usize, j: usize, nums: &Vec<i32>, dp: &mut Vec<Vec<i32>>) -> i32 {
            if dp[i][j] > 0 {
                return dp[i][j];
            }

            let m = if i > j {
                0
            } else if i == j {
                let left = if i == 0 { 1 } else { nums[i - 1] };
                let right = if j == dp.len() - 1 { 1 } else { nums[j + 1] };
                left * nums[i] * right
            } else {
                let mut m = 0;
                let left = if i == 0 { 1 } else { nums[i - 1] };
                let right = if j == dp.len() - 1 { 1 } else { nums[j + 1] };
                for k in i..=j {
                    let leftm = if k > 0 { core(i, k - 1, nums, dp) } else { 0 };
                    let rightm = if k < dp.len() - 1 { core(k + 1, j, nums, dp) } else { 0 };
                    let mm = leftm + left * nums[k] * right + rightm;
                    m = max(m, mm);
                }
                m
            };
            dp[i][j] = m;
            m
        }

        core(0, n - 1, &nums, &mut dp)
    }
}