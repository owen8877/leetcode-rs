pub fn missing_number(mut nums: Vec<i32>) -> i32 {
    nums.push(-1);
    for i in 0..nums.len() {
        while nums[i] >= 0 && nums[i] != i as i32 {
            let a = nums[i] as usize;
            nums.swap(i, a);
        }
    }
    for i in 0..nums.len() {
        if nums[i] == -1 {
            return i as i32;
        }
    }
    unreachable!()
}