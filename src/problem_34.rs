pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() == 0 {
        return vec![-1, -1];
    }

    use std::cmp::Ordering::*;

    fn left(nums: &[i32], target: i32) -> Option<usize> {
        match nums.len() {
            0 => None,
            1 => if nums[0] == target { Some(0) } else { None },
            n => {
                if nums[0] == target {
                    return Some(0)
                }
                let index = (n-1) / 2;
                match nums[index].cmp(&target) {
                    Less => left(&nums[index+1..], target).map(|x| x+index+1),
                    Equal => left(&nums[..=index], target),
                    Greater => left(&nums[..index], target),
                }
            }
        }
    }

    fn right(nums: &[i32], target: i32) -> Option<usize> {
        match nums.len() {
            0 => None,
            1 => if nums[0] == target { Some(0) } else { None },
            n => {
                if nums[n-1] == target {
                    return Some(n-1)
                }
                let index = n / 2;
                match nums[index].cmp(&target) {
                    Less => right(&nums[index+1..], target).map(|x| x+index+1),
                    Equal => right(&nums[index..], target).map(|x| x+index),
                    Greater => right(&nums[..index], target),
                }
            }
        }
    }

    match left(&nums[..], target) {
        None => vec![-1, -1],
        Some(l) => vec![l as i32, right(&nums[..], target).unwrap() as i32],
    }
}

#[test]
fn test_search_range() {
    assert_eq!(search_range(vec![5,7,7,8,8,10], 8), vec![3, 4]);
    assert_eq!(search_range(vec![5,7,7,8,8,10], 6), vec![-1, -1]);
}