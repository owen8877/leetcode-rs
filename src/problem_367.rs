pub fn is_perfect_square(num: i32) -> bool {
    use std::cmp::Ordering::*;

    let num = num as i64;
    if num == 1 {
        return true;
    }

    let mut left = 1;
    let mut right = num;

    while right > left + 1 {
        let mid = (left + right) / 2;
        match (mid * mid).cmp(&num) {
            Greater => {
                right = mid;
            }
            Less => {
                left = mid;
            }
            Equal => {
                return true;
            }
        }
    }
    false
}