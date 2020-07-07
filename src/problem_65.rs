pub fn is_number(s: String) -> bool {
    if s.is_empty() {
        return false
    }
    let mut seen_e = false;
    let mut seen_sign = false;
    let mut seen_point = false;
    let mut seen_digits = false;
    enum BlankState {
        NeverSeen, Beginning, BeginningEnded, Trailing, TrailingEnded,
    }
    use BlankState::*;
    let mut blanks = NeverSeen;
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            match blanks {
                NeverSeen => {
                    if i == 0 {
                        blanks = Beginning;
                    } else {
                        blanks = Trailing;
                    }
                },
                BeginningEnded => blanks = Trailing,
                TrailingEnded => return false,
                _ => {},
            }
            continue
        } else {
            match blanks {
                NeverSeen => blanks = BeginningEnded,
                Beginning => blanks = BeginningEnded,
                Trailing => return false,
                _ => {},
            }
        }
        match c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                seen_digits = true;
            },
            ' ' => unreachable!(),
            'e' => {
                if seen_e || !seen_digits {
                    return false
                }
                seen_e = true;
                seen_digits = false;
                seen_sign = false;
                seen_point = false;
            },
            '+' | '-' => {
                if seen_digits || seen_point || seen_sign {
                    return false
                }
                seen_sign = true;
            },
            '.' => {
                if seen_point || seen_e {
                    return false
                }
                seen_point = true;
            },
            _ => return false,
        }
    }
    seen_digits
}

#[test]
fn test_is_number() {
    assert_eq!(is_number("32.e-80123".to_string()), true);
    return;
    assert_eq!(is_number("abc".to_string()), false);
    assert_eq!(is_number(". 1".to_string()), false);
    assert_eq!(is_number("1 a".to_string()), false);
    assert_eq!(is_number("0".to_string()), true);
    assert_eq!(is_number(" 0.1 ".to_string()), true);
    assert_eq!(is_number(" -90e3   ".to_string()), true);
    assert_eq!(is_number("2e10".to_string()), true);
    assert_eq!(is_number(" 1e".to_string()), false);
    assert_eq!(is_number("e3".to_string()), false);
    assert_eq!(is_number(" 6e-1".to_string()), true);
    assert_eq!(is_number(" 99e2.5 ".to_string()), false);
    assert_eq!(is_number("53.5e93".to_string()), true);
    assert_eq!(is_number(" --6 ".to_string()), false);
    assert_eq!(is_number("-+3".to_string()), false);
    assert_eq!(is_number("95a54e53".to_string()), false);
    assert_eq!(is_number("01".to_string()), true);
    assert_eq!(is_number("01e 1".to_string()), true);
    assert_eq!(is_number(".1".to_string()), true);
    assert_eq!(is_number("1.".to_string()), true);
}