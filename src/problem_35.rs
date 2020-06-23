pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 0 {
        return 0
    }

    for (i, num) in nums.iter().enumerate() {
        if num as &i32 >= &target {
            return i as i32
        }
    }

    nums.len() as i32
}

#[test]
fn test_search_insert() {
    assert_eq!(search_insert(vec![1,3,5,6], 7), 4);
    assert_eq!(search_insert(vec![1,3,5,6], 0), 0);
}