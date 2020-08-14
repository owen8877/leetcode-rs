pub fn find_min(nums: Vec<i32>) -> i32 {
    use std::cmp::*;
    fn find_min_rotated(nums: &[i32]) -> i32 {
        match nums.len() {
            1 => nums[0],
            2 => min(nums[0], nums[1]),
            k => {
                let left = nums[0];
                let mid = nums[k/2];
                let right = nums[k-1];

                if left > mid {
                    find_min_rotated(&nums[..=k/2])
                } else if left == mid {
                    if mid > right {
                        find_min_rotated(&nums[k/2..])
                    } else if mid == right {
                        min(find_min_rotated(&nums[..=k/2]), find_min_rotated(&nums[k/2..]))
                    } else {
                        left
                    }
                } else {
                    if left >= right {
                        find_min_rotated(&nums[k/2..])
                    } else {
                        left
                    }
                }
            }
        }
    }

    find_min_rotated(nums.as_slice())
}
