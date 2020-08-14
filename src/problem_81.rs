pub fn search(nums: Vec<i32>, target: i32) -> bool {
    use std::cmp::Ordering::*;
    let n = nums.len();
    if n == 0 {
        return false
    }
    if n == 1 {
        return nums[0] == target
    }

    fn core(nums: &[i32], target: i32) -> bool {
        match nums.len() {
            0 => false,
            1 => nums[0] == target,
            n => {
                match nums[n / 2].cmp(&target) {
                    Less => core(&nums[n / 2 + 1..], target),
                    Equal => true,
                    Greater => core(&nums[..n / 2], target),
                }
            },
        }
    }

    let mut d = n - 1;
    for i in 0..n - 1 {
        if nums[i] > nums[i + 1] {
            d = i;
        }
    }
    core(&nums[..d + 1], target) || core(&nums[d + 1..], target)
}