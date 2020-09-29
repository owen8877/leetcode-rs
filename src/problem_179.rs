pub fn largest_number(nums: Vec<i32>) -> String {
    let mut nums = nums.into_iter().map(|num| format!("{}", num)).collect::<Vec<String>>();
    nums.sort_by(|a, b| (b.to_string() + a).cmp(&(a.to_string() + b)));
    if nums[0] == "0" {
        nums[0].clone()
    } else {
        nums.into_iter().fold("".to_string(), |acc, x| acc + x.as_str())
    }
}