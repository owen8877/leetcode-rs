pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.len() == 0 {
        return vec![];
    }
    let mut result = vec![];
    let mut start = nums[0];
    let mut last = nums[0];
    for num in nums {
        if num > last + 1 {
            result.push(if start == last {
                format!("{}", start)
            } else {
                format!("{}->{}", start, last)
            }.to_string());
            last = num;
            start = last;
        } else {
            last = num;
        }
    }
    result.push(if start == last {
        format!("{}", start)
    } else {
        format!("{}->{}", start, last)
    }.to_string());
    result
}