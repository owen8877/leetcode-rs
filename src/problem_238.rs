pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    if n == 1 {
        return vec![1];
    }
    let mut answer = vec![1; n];
    for i in 1..n {
        answer[i] = answer[i - 1] * nums[i - 1];
    }
    let mut r = 1;
    for i in 1..n {
        r *= nums[n - i];
        answer[n - i - 1] *= r;
    }
    answer
}