pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut p0 = 0;
    let mut p1 = 0;
    let mut p2 = 0;
    for i in 0..nums.len() {
        match nums[i] {
            0 => {
                nums[p2] = 2;
                p2 += 1;
                nums[p1] = 1;
                p1 += 1;
                nums[p0] = 0;
                p0 += 1;
            },
            1 => {
                nums[p2] = 2;
                p2 += 1;
                nums[p1] = 1;
                p1 += 1;
            },
            _ => {
                nums[p2] = 2;
                p2 += 1;
            },
        }
    }
}

#[test]
fn test_sort_colors() {
    let mut arr = vec![2, 0, 0, 1, 2, 0, 1];
    sort_colors(&mut arr);
    assert_eq!(arr, vec![0, 0, 0, 1, 1, 2, 2]);
}