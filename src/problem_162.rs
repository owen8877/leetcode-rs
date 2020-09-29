pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    match nums.len() {
        1 => 0,
        n => {
            if nums[0] > nums[1] {
                0
            } else {
                for i in 1..n-1 {
                    if nums[i] > nums[i-1] && nums[i] > nums[i+1] {
                        return i as i32
                    }
                }
                n as i32 - 1
            }
        }
    }
}