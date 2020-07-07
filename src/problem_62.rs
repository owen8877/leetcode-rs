pub fn unique_paths(m: i32, n: i32) -> i32 {
    let (m, n) = if m > n {
        (m as i64, n as i64)
    } else {
        (n as i64, m as i64)
    };
    (1..(n as i64)).fold(1, |acc, x| acc * (m+n-1-x) / x) as i32
}