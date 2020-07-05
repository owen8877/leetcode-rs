pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    use std::cmp::max;

    let n = nums.len();
    if n == 0 {
        return 0
    } else if n == 1 {
        return nums[0]
    }

    let mut is_in_pos = false;
    let mut recent_pos_sum = 0;
    let mut recent_neg_sum = 0;
    let mut largest_sum = 0;
    let mut meet_pos = false;
    let mut largest_num = nums[0];

    for x in nums {
        if x < 0 {
            is_in_pos = false;
            recent_neg_sum += x;
            if recent_neg_sum + recent_pos_sum < 0 {
                recent_pos_sum = 0;
                recent_neg_sum = 0;
            }
        } else {
            is_in_pos = true;
            meet_pos = true;
            recent_pos_sum += x;
            largest_sum = max(largest_sum, recent_neg_sum + recent_pos_sum);
        }
        largest_num = max(largest_num, x);
    }

    if meet_pos {
        largest_sum
    } else {
        largest_num
    }
}

#[test]
fn test_max_sub_array() {
    assert_eq!(max_sub_array(vec![]), 0);
    assert_eq!(max_sub_array(vec![-1, 1, 2, -1]), 3);
    assert_eq!(max_sub_array(vec![-1, 1, 2, -1, -3, 4]), 4);
    assert_eq!(max_sub_array(vec![-1, 1, 2, -1, -3, 1]), 3);
    assert_eq!(max_sub_array(vec![-1, -2, -3, -4, -5]), -1);
}