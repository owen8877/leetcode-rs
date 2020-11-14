pub fn wiggle_sort(nums: &mut Vec<i32>) {
    nums.sort();
    let n = nums.len();
    let mut result = vec![0; n];
    if n % 2 == 0 {
        for i in 0..n / 2 {
            result[n - 2 - 2 * i] = nums[i];
            result[n - 1 - 2 * i] = nums[n / 2 + i];
        }
    } else {
        for i in 0..n / 2 {
            result[n - 1 - 2 * i] = nums[i];
            result[n - 2 - 2 * i] = nums[n / 2 + i + 1];
        }
        result[0] = nums[n / 2];
    }
    for i in 0..n {
        nums[i] = result[i];
    }
}