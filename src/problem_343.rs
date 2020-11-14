pub fn integer_break(n: i32) -> i32 {
    match n {
        2 => 1,
        3 => 2,
        4 => 4,
        5 => 6,
        n if n % 3 == 0 => 3i32.pow((n / 3) as u32),
        n if n % 3 == 1 => 3i32.pow((n / 3) as u32 - 1) * 4,
        _ => 3i32.pow((n / 3) as u32) * 2,
    }
}