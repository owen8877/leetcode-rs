pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() <= 1 {
        return nums.len() as i32
    }
    let mut pointer = 1;
    for i in 1..nums.len() {
        if nums[i] > nums[pointer-1] {
            nums[pointer] = nums[i];
            pointer += 1;
        }
    }

    pointer as i32
}

#[test]
fn test_remove_duplicates() {
    let mut vec1 = vec![0, 0, 1];
    let n1 = remove_duplicates(&mut vec1);
    assert_eq!(&vec1[..n1 as usize], &[0, 1]);
    assert_eq!(n1, 2);

    let mut vec2 = vec![0,0,1,1,1,2,2,3,3,4];
    let n2 = remove_duplicates(&mut vec2);
    assert_eq!(&vec2[..n2 as usize], &[0, 1, 2, 3, 4]);
    assert_eq!(n2, 5);
}