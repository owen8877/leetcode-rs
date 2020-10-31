// TODO: Needs improvement
pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![0; n];
    for i in 0..n {
        for j in i + 1..n {
            if nums[i] > nums[j] {
                result[i] += 1;
            }
        }
    }
    result
}