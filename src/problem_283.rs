pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut cursor = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums[cursor] = nums[i];
            cursor += 1;
        }
    }
    for j in cursor..nums.len() {
        nums[j] = 0;
    }
}