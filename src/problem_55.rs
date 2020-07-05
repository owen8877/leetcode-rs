pub fn can_jump(nums: Vec<i32>) -> bool {
    let n = nums.len();
    if n == 0 {
        return false
    } else if n == 1 {
        return true
    }
    if nums[0] as usize >= n-1 {
        return true
    }
    let mut leftmost = n - 1;
    for j in (0..n-1).rev() {
        if nums[j] as usize + j >= leftmost {
            leftmost = j;
        }
    }

    leftmost == 0
}