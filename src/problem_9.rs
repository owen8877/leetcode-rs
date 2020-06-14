pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    if 0 <= x && x < 10 {
        return true;
    }

    let mut digits = Vec::<u8>::new();
    let mut xx = x;
    while xx > 0 {
        digits.push((xx % 10) as u8);
        xx /= 10;
    }

    for d in digits.iter().zip(digits.iter().rev()) {
        if d.0 != d.1 {
            return false;
        }
    }

    true
}

#[test]
fn test_is_palindrome() {
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(-121), false);
    assert_eq!(is_palindrome(123454321), true);
    assert_eq!(is_palindrome(10), false);
    assert_eq!(is_palindrome(120), false);
}