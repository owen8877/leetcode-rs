pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut pointer = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[pointer] = nums[i];
            pointer += 1;
        }
    }

    pointer as i32
}

#[test]
fn test_remove_element() {
    let mut vec1 = vec![0, 0, 1];
    let n1 = remove_element(&mut vec1, 0);
    assert_eq!(&vec1[..n1 as usize], &[1]);
    assert_eq!(n1, 1);

    let mut vec2 = vec![0,1,2,2,3,0,4,2];
    let n2 = remove_element(&mut vec2, 2);
    assert_eq!(&vec2[..n2 as usize], &[0, 1, 3, 0, 4]);
    assert_eq!(n2, 5);
}