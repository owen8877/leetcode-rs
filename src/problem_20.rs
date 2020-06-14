pub fn is_valid(s: String) -> bool {
    let n = s.len();
    if n == 0 {
        return true
    }
    if n % 2 == 1 {
        return false
    }

    let mut previous_pos = vec![0; n];
    let chars: Vec<char> = s.chars().collect();
    let mut last_position = 0;
    let mut counter = 0;

    for i in 0..n {
        let c = chars[i];
        match c {
            '{' | '[' | '(' => {
                previous_pos[i] = last_position;
                last_position = i;
                counter += 1;
            },
            '}' | ']' | ')' => {
                let d = chars[last_position];
                if !(d == '(' && c == ')') && !(d == '[' && c == ']') && !(d == '{' && c == '}') {
                    return false
                }
                last_position = previous_pos[last_position];
                counter -= 1;
            },
            _ => panic!("Didn't expect {}!", c),
        }
    }

    counter == 0
}

#[test]
fn test_is_valid() {
    // assert_eq!(is_valid("()".to_string()), true);
    // assert_eq!(is_valid("()[]{}".to_string()), true);
    // assert_eq!(is_valid("(]".to_string()), false);
    // assert_eq!(is_valid("([)]".to_string()), false);
    assert_eq!(is_valid("((".to_string()), false);
}