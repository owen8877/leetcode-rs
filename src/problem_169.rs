pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut maj = 0;
    let mut counter = 0;
    for num in nums {
        if counter == 0 {
            maj = num;
            counter = 1;
        } else {
            if num == maj {
                counter += 1;
            } else {
                counter -= 1;
            }
        }
    }
    maj
}