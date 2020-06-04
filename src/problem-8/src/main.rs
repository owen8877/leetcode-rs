pub fn my_atoi(str: String) -> i32 {
    let v: Vec<char> = str.chars().collect();

    let mut num = 0;
    const INT_MAX: i32 = - ((-1 << 31) + 1);
    const INT_MIN: i32 = -1 << 31;
    let mut pos = true;
    let mut leading_white_space = true;
    let mut has_sign = false;
    let mut has_digit = false;
    for c in v.iter() {
        match c {
            ' ' => {
                if !leading_white_space || has_digit {
                    return num
                }
                if has_sign {
                    return 0
                }
            },
            '+' => {
                leading_white_space = false;
                if has_digit {
                    return num;
                }
                if has_sign {
                    return 0;
                } else {
                    pos = true;
                    has_sign = true;
                }
            },
            '-' => {
                leading_white_space = false;
                if has_digit {
                    return num;
                }
                if has_sign {
                    return 0;
                } else {
                    pos = false;
                    has_sign = true;
                }
            },
            '0'..='9' => {
                has_digit = true;
                let ci32 = c.clone() as i32 - '0' as i32;
                if pos {
                    let num64 = num as i64 * 10 + ci32 as i64;
                    if num64 >= INT_MAX as i64 {
                        return INT_MAX
                    } else {
                        num = num64 as i32;
                    }
                } else {
                    let num64 = num as i64 * 10 - ci32 as i64;
                    if num64 <= INT_MIN as i64 {
                        return INT_MIN
                    } else {
                        num = num64 as i32;
                    }
                }
            },
            _ => return num,
        }
    }

    num
}

#[test]
fn test_my_atoi() {
    assert_eq!(my_atoi(String::from("42")), 42);
    assert_eq!(my_atoi(String::from("    -42")), -42);
    assert_eq!(my_atoi(String::from("42 as a word")), 42);
    assert_eq!(my_atoi(String::from("-91283472332")), -2147483648);
    assert_eq!(my_atoi(String::from("+-2")), 0);
    assert_eq!(my_atoi(String::from("2147483646")), 2147483646);
    assert_eq!(my_atoi(String::from("0-1")), 0);
}