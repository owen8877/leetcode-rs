pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashMap;
    use std::cmp::Ordering::*;
    let mut num_freq = HashMap::<i32, u32>::new();
    for num in nums {
        match num_freq.get(&num) {
            None => num_freq.insert(num, 1),
            Some(k) => num_freq.insert(num, k+1),
        };
    }

    let mut result = Vec::<Vec<i32>>::new();
    let mut num_unique_s: Vec<i32> = num_freq.keys().map(|x| x.clone()).collect();
    num_unique_s.sort();

    fn find(vec: &[i32], y: &i32) -> bool{
        if vec.len() == 0 {
            return false
        }
        if vec.len() == 1 {
            return vec[0] == *y
        }

        let mid_index = vec.len() / 2;
        let mid = &vec[mid_index];
        match mid.cmp(y) {
            Equal => true,
            Less => find(&vec[mid_index+1..], y),
            Greater => find(&vec[..mid_index], y),
        }
    }

    for i in 0..num_unique_s.len() {
        let sum = num_unique_s[i];
        for j in i..num_unique_s.len() {
            let x = num_unique_s[j];
            let y = - sum - x;
            if !find(&num_unique_s[j..], &y) {
                continue
            }
            if sum == x {
                if sum == y {
                    if num_freq.get(&sum).unwrap() < &3 {
                        continue
                    }
                } else {
                    if num_freq.get(&sum).unwrap() < &2 {
                        continue
                    }
                }
            } else {
                if sum == y {
                    if num_freq.get(&sum).unwrap() < &2 {
                        continue
                    }
                } else if x == y {
                    if num_freq.get(&x).unwrap() < &2 {
                        continue
                    }
                }
            }
            result.push(vec![sum, x, y]);
        }
    }
    result
}

pub fn three_sum_2(nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashSet;
    use std::cmp::Ordering::*;

    let mut result = HashSet::<Vec<i32>>::new();
    let mut nums = nums;
    nums.sort();

    for i in 0..nums.len() {
        let mut j = i+1;
        let mut k = nums.len()-1;
        while j < k {
            match (-nums[i]).cmp(&(nums[j] + nums[k])) {
                Equal => {
                    result.insert(vec![nums[i], nums[j], nums[k]]);
                    j += 1;
                    k -= 1;
                },
                Less => {
                    k -= 1;
                },
                Greater => {
                    j += 1;
                },
            }
        }
    }

    result.into_iter().collect()
}

#[test]
fn test_three_sum() {
    assert_eq!(three_sum_2(vec![-1, 0, 1, 2, -1, -4]), vec![
        vec![-1, -1, 2],
        vec![-1, 0, 1],
    ]);
    assert_eq!(three_sum_2(vec![-2, 0, 1, 1, 2]), vec![
        vec![-2, 0, 2],
        vec![-2, 1, 1],
    ]);
}