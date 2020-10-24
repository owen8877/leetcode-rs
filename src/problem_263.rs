pub fn is_ugly(mut num: i32) -> bool {
    if num <= 0 {
        false
    } else if num <= 6 {
        true
    } else {
        loop {
            if num % 2 == 0 {
                num /= 2;
            } else if num % 3 == 0 {
                num /= 3;
            } else if num % 5 == 0 {
                num /= 5;
            } else {
                break;
            }
        }
        num == 1
    }
}