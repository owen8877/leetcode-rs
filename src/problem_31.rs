pub fn next_permutation(nums: &mut Vec<i32>) {
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

#[test]
fn test_next_permutation() {
    let mut arr = vec![1, 2, 3];
    next_permutation(arr.as_mut());
    assert_eq!(arr, vec![1, 3, 2]);

    arr = vec![1, 3, 4, 2];
    next_permutation(arr.as_mut());
    assert_eq!(arr, vec![1, 4, 2, 3]);

    arr = vec![3, 1, 4, 2];
    next_permutation(arr.as_mut());
    assert_eq!(arr, vec![3, 2, 1, 4]);

    arr = vec![4, 2, 10, 5, 3, 1];
    next_permutation(arr.as_mut());
    assert_eq!(arr, vec![4, 3, 1, 2, 5, 10]);
}