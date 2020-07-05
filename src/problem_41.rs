pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    'outer: for i in 0..nums.len() {
        loop {
            let j = nums[i];
            if j < 1 {
                continue 'outer
            }
            let j = j as usize;
            if j > nums.len() {
                continue 'outer
            }
            if j as i32 == nums[j-1] {
                continue 'outer
            }
            let tmp = nums[j-1];
            nums[j-1] = j as i32;
            nums[i] = tmp;
        }
    }

    for i in 0..nums.len() {
        if i as i32 +1 != nums[i] {
            return i as i32 +1
        }
    }
    (nums.len()+1) as i32
}