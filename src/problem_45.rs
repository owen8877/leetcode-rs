pub fn jump_naive(nums: Vec<i32>) -> i32 {
    use std::cmp::min;
    let n = nums.len();
    if n == 1 {
        return 0
    } else if n == 2 {
        return 1
    }

    let mut min_jump = vec![0; n];
    for i in (0..n-1).rev() {
        min_jump[i] = *&min_jump[i+1..=min(i+nums[i] as usize, n-1)].iter().fold(n as i32, |acc, x| min(acc, *x)) + 1;
    }
    min_jump[0]
}

// Reference: https://leetcode.com/problems/jump-game-ii/discuss/706435/C%2B%2B-o(n)-solution
// The idea is to maintain a range (start, end] s.t. every position in this range needs an exact s steps to reach
pub fn jump(nums: Vec<i32>) -> i32{
    use std::cmp::max;
    let n = nums.len();
    if n == 1 {
        return 0
    } else if n == 2 {
        return 1
    } else if nums[0] >= n as i32 - 1 {
        return 1
    }

    let mut end = nums[0];
    let mut next_end = nums[0];
    let mut s = 1;
    for j in 1..n {
        next_end = max(next_end, j as i32 + nums[j]);
        if next_end >= n as i32 - 1 {
            return s+1
        }
        if j as i32 == end {
            end = next_end;
            s += 1;
        }
    }
    s
}

#[test]
fn test_jump() {
    assert_eq!(jump(vec![3, 2, 1]), 1);
    assert_eq!(jump(vec![2, 3, 1, 1, 4]), 2);
    assert_eq!(jump(vec![1, 2, 3]), 2);
}