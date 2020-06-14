pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    use std::cmp::Ordering::*;
    let mut nums = nums;
    nums.sort();
    let mut closest = nums[0] + nums[nums.len()/2] + nums[nums.len()-1];
    let mut closest_diff = (closest-target).abs();
    if closest_diff == 0 {
        return target;
    }

    for i in 0..nums.len() {
        let mut j = i+1;
        let mut k = nums.len() - 1;
        while j < k {
            let sum = nums[i]+nums[j]+nums[k];
            let diff = (sum-target).abs();
            if diff < closest_diff {
                closest = sum;
                closest_diff = diff;
            }
            match target.cmp(&(sum)) {
                Equal => return target,
                Less => {
                    k -= 1;
                },
                Greater => {
                    j += 1;
                },
            }
        }
    }

    closest
}

#[test]
fn test_three_sum_closest() {
    assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
}