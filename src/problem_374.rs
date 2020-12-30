unsafe fn guessNumber(n: i32) -> i32 {
    let mut l = 1;
    let mut r = n as i64 + 1;
    while r - l > 1 {
        let mut mid = (l + r) / 2;
        match guess(mid as i32) {
            -1 => r = mid,
            1 => l = mid,
            _ => return mid as i32
        }
    }
    l as i32
}