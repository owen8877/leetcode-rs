pub fn count_digit_one(n: i32) -> i32 {
    use std::cmp::*;
    let n = n as i64;
    let mut count = 0;
    let mut i = 1;
    while i <= n {
        count += (n / (i * 10)) * i + min(max(n % (10 * i) - i + 1, 0), i);
        i *= 10;
    }
    count as i32
}