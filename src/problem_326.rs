pub fn is_power_of_three(n: i32) -> bool {
    if n == 1 {
        true
    } else if n % 3 != 0 || n <= 0 {
        false
    } else {
        is_power_of_three(n / 3)
    }
}