pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0
    }
    let mut write = 0;
    let mut cache = nums[0];
    let mut count = 0;
    for read in 0..nums.len() {
        if nums[read] == cache {
            if count < 2 {
                count += 1;
                nums[write] = cache;
                write += 1;
            }
        } else {
            cache = nums[read];
            count = 1;
            nums[write] = cache;
            write += 1;
        }
    }
    write as i32
}