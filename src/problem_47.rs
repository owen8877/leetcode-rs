// Needs improvement
pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn next_permutation(nums: &mut Vec<i32>) {
        fn swap(v: &mut Vec<i32>, i: usize, j: usize) {
            let tmp = v[i];
            v[i] = v[j];
            v[j] = tmp;
        }

        match nums.len() {
            0 | 1 => return,
            2 => {
                swap(nums, 0, 1);
            },
            n => {
                let mut bpointer = n-1;
                while bpointer > 0 && nums[bpointer] <= nums[bpointer-1] {
                    bpointer -= 1;
                }
                if bpointer == 0 {
                    for i in 0..nums.len()/2 {
                        swap(nums, i, nums.len()-1-i);
                    }
                } else {
                    for j in bpointer..(nums.len()+bpointer)/2 {
                        swap(nums, j, nums.len()-1-j+bpointer);
                    }
                    for j in bpointer..nums.len() {
                        if nums[j] > nums[bpointer-1] {
                            swap(nums, j, bpointer-1);
                            return
                        }
                    }

                }
            }
        }
    }

    let mut nums = nums;
    nums.sort();
    let m = nums.iter().fold((1, nums[0]-1, 1), |acc, x| {
        if acc.1 == *x {
            (acc.0*(acc.2+1), acc.1, acc.2+1)
        } else {
            (acc.0, *x, 1)
        }
    }).0;
    let mut result = vec![];
    for i in 0..(1..=nums.len()).fold(1, |acc, x| acc * x)/m {
        result.push(nums.clone());
        next_permutation(&mut nums);
    }
    result
}