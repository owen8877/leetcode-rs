pub fn nth_ugly_number(n: i32) -> i32 {
    use std::cmp::min;
    let n = n as usize;
    let mut nums = vec![1; n];
    let mut l1 = 0;
    let mut l2 = 0;
    let mut l3 = 0;
    for i in 1..n {
        nums[i] = min(nums[l1] * 2, min(nums[l2] * 3, nums[l3] * 5));
        if nums[i] == nums[l1] * 2 {
            l1 += 1;
        }
        if nums[i] == nums[l2] * 3 {
            l2 += 1;
        }
        if nums[i] == nums[l3] * 5 {
            l3 += 1;
        }
    }
    nums[n - 1]
}