pub fn rob(nums: Vec<i32>) -> i32 {
    use std::cmp::*;
    fn core(nums: &[i32]) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0
        } else if n == 1 {
            return nums[0]
        }
        let mut s1 = nums[0];
        let mut s2 = max(nums[0], nums[1]);
        for i in 2..n {
            let s_new = max(s2, s1+nums[i]);
            s1 = s2;
            s2 = s_new;
        }
        s2
    }
    match nums.len() {
        1 => nums[0],
        n => max(core(&nums[..n-1]), core(&nums[1..])),
    }
}