pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    if nums[left] < nums[right] {
        return nums[left]
    }
    while right > left + 1 {
        let mid = (right+left) / 2;
        if nums[left] < nums[mid] {
            left = mid;
        } else {
            right = mid;
        }
    }
    nums[right]
}