pub fn is_power_of_four(n: i32) -> bool {
    match n {
        n if n <= 0 => false,
        1 => true,
        n if n % 4 == 0 => Self::is_power_of_four(n / 4),
        _ => false,
    }
}