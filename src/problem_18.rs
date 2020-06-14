pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    use std::collections::{HashMap, HashSet};
    if nums.len() < 4 {
        return vec![]
    }

    let mut two_sums = HashMap::<i32, Vec<(usize, usize)>>::new();
    let n = nums.len();
    for i in 0..n {
        for j in 0..i {
            let sum = nums[i] + nums[j];
            match two_sums.get_mut(&sum) {
                None => {
                    two_sums.insert(sum, vec![(j, i)]);
                },
                Some(vec) => {
                    vec.push((j, i));
                },
            }
        }
    }

    let mut results = HashSet::<Vec<i32>>::new();
    for sum in two_sums.keys() {
        match two_sums.get(&(&target-sum)) {
            None => {
                continue
            },
            Some(arr2) => {
                let arr1 = two_sums.get(sum).unwrap();
                for pair1 in arr1 {
                    for pair2 in arr2 {
                        let mut vec_index = vec![pair1.0, pair1.1, pair2.0, pair2.1];
                        vec_index.sort();
                        let mut vec = vec![nums[pair1.0], nums[pair1.1], nums[pair2.0], nums[pair2.1]];
                        vec.sort();
                        if vec_index[0] < vec_index[1] && vec_index[1] < vec_index[2] && vec_index[2] < vec_index[3] {
                            results.insert(vec);
                        }
                    }
                }
            },
        }
    }

    results.into_iter().collect()
}

#[test]
fn test_four_sum() {
    assert_eq!(four_sum(vec![1, 0, -1, 0, -2, 2], 0), vec![
        vec![-1,  0, 0, 1],
        vec![-2, -1, 1, 2],
        vec![-2,  0, 0, 2],
    ]);
}
