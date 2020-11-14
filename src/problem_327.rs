pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
    use std::collections::*;
    use std::ops::Bound::Included;

    let mut cumsums = BTreeMap::new();
    let mut cumsum = 0;
    let mut count = 0;
    let lower = lower as i64;
    let upper = upper as i64;
    cumsums.insert(cumsum, 1);
    for num in nums {
        let num = num as i64;
        cumsum += num;
        count += cumsums.range((Included(&(cumsum - upper)), Included(&(cumsum - lower)))).fold(0, |acc, x| acc + x.1);
        *cumsums.entry(cumsum).or_insert(0) += 1;
    }
    count as i32
}