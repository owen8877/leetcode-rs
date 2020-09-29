pub fn max_product(nums: Vec<i32>) -> i32 {
    use std::cmp::*;
    let mut max_pos_cont = nums[0];
    let mut max_neg_cont = nums[0];
    let mut max_pos = max_pos_cont;
    for i in 1..nums.len() {
        let num = nums[i];
        let max_pos_cont_tmp = max(max(max_pos_cont*num, max_neg_cont*num), num);
        max_neg_cont = min(min(max_pos_cont*num, max_neg_cont*num), num);
        max_pos_cont = max_pos_cont_tmp;
        max_pos = max(max_pos, max_pos_cont);
    }
    max_pos
}