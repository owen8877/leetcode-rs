pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    for (i, x) in nums.iter().enumerate() {
        for j in 0..i {
            if *x == nums[j] {
                return *x
            }
        }
    }
    0
}