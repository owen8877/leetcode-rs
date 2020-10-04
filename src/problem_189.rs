pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    fn reverse(nums: &mut Vec<i32>, left: usize, right: usize) {
        let mut counter = 0;
        while left + counter < right - counter {
            nums.swap(left+counter, right-counter);
            counter += 1;
        }
    }

    let n = nums.len();
    let k = (k as usize) % n;
    if n > 1 {
        reverse(nums, 0, n-1);
    }
    if k > 0 {
        reverse(nums, 0, k-1);
    }
    if n > 1 {
        reverse(nums, k, n-1);
    }
}