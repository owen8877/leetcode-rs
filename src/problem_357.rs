pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
    if n == 0 {
        return 1;
    } else if n == 1 {
        return 10;
    }
    let mut count = 10;
    let mut cummulative = 9;
    for i in 0..(n as usize) - 1 {
        cummulative *= 9 - i;
        count += cummulative;
    }
    count as i32
}