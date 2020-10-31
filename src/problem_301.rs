pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
    let mut left = 0;
    let mut right = 0;

    for c in s.chars() {
        match c {
            '(' => left += 1,
            ')' => if left == 0 {
                right += 1;
            } else {
                left -= 1;
            },
            _ => {}
        }
    }

    use std::collections::*;
    let mut result = HashSet::new();
    let mut expr = vec![];
    let s: Vec<char> = s.chars().collect();

    fn core(s: &[char], index: usize, left_count: usize, right_count: usize, left_rem: usize, right_rem: usize, expr: &mut Vec<u8>, result: &mut HashSet<String>) {
        if index == s.len() {
            if left_rem == 0 && right_rem == 0 {
                result.insert(String::from_utf8_lossy(expr).to_string());
            }
            return;
        }

        if s[index] == '(' && left_rem > 0 {
            core(s, index + 1, left_count, right_count, left_rem - 1, right_rem, expr, result);
        } else if s[index] == ')' && right_rem > 0 {
            core(s, index + 1, left_count, right_count, left_rem, right_rem - 1, expr, result);
        }

        expr.push(s[index] as u8);

        match s[index] {
            '(' => {
                core(s, index + 1, left_count + 1, right_count, left_rem, right_rem, expr, result);
            }
            ')' => {
                if left_count > right_count {
                    core(s, index + 1, left_count, right_count + 1, left_rem, right_rem, expr, result);
                }
            }
            _ => {
                core(s, index + 1, left_count, right_count, left_rem, right_rem, expr, result);
            }
        }

        expr.pop();
    }

    core(&s, 0, 0, 0, left, right, &mut expr, &mut result);
    result.iter().map(|s| s.clone()).collect()
}