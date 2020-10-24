// Reference: https://leetcode.com/problems/single-number-iii/discuss/750622/Python-4-Lines-O(n)-time-O(1)-space-explained
pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
    let sum = nums.iter().fold(0, |acc, x| acc ^ *x);
    let nz = (sum & (sum - 1)) ^ sum;
    let num1 = nums.iter().filter(|x| *x & nz != 0).fold(0, |acc, x| acc ^ *x);
    vec![num1, sum ^ num1]
}