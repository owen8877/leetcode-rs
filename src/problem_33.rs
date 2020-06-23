pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    use std::cmp::Ordering::*;
    let n = nums.len();
    if n == 0 {
        return -1
    }
    if n == 1 {
        return if nums[0] == target { 0 } else { -1 }
    }

    fn core(nums: &[i32], target: i32) -> Option<i32> {
        match nums.len() {
            0 => None,
            1 => if nums[0] == target { Some(0) } else { None },
            n => {
                match nums[n/2].cmp(&target) {
                    Less => core(&nums[n/2+1..], target).map(|x| x+(n/2+1) as i32),
                    Equal => Some((n/2) as i32),
                    Greater => core(&nums[..n/2], target),
                }
            },
        }
    }

    let mut d = n-1;
    for i in 0..n-1 {
        if nums[i] > nums[i+1] {
            d = i;
        }
    }
    match core(&nums[..d+1], target) {
        None => match core(&nums[d+1..], target) {
            None => -1,
            Some(m) => m + (d+1) as i32,
        },
        Some(n) => n,
    }
}

#[test]
fn test_search() {
    assert_eq!(search(vec![], 3), -1);
    assert_eq!(search(vec![4,5,6,7,0,1,2], 3), -1);
    assert_eq!(search(vec![4,5,6,7,0,1,2], 0), 4);
    assert_eq!(search(vec![1, 3, 5], 5), 2);
}