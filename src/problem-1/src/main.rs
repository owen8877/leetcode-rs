fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    vec![0, 1]
}

#[test]
fn test_two_sum() {
    let nums = vec![2, 7, 10, 11];
    let target = 17;
    assert_eq!(two_sum(nums, target), [1, 2]);
}