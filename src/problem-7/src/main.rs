pub fn reverse(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }
    let pos = x > 0;
    let mut y = if pos { x } else { -x };

    let mut ry = 0;
    while y > 0 {
        let ry64 = ry as i64 * 10 + (y % 10) as i64;
        if ry64 > (1 << 31) - 1 {
            return 0;
        }
        ry = ry64 as i32;
        y /= 10;
    }

    if pos { ry } else { -ry }
}

#[test]
fn test_reverse() {
    // assert_eq!(reverse(120), 21);
    // assert_eq!(reverse(123), 321);
    // assert_eq!(reverse(-123), -321);
    // assert_eq!(reverse(1534236469), 0);
    assert_eq!(reverse(-2147483412), -2143847412);
}