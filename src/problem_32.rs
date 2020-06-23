pub fn longest_valid_parentheses(s: String) -> i32 {
    use std::cmp::max;
    if s.len() < 2 {
        return 0
    }
    if s.len() == 2 {
        if s == "()".to_string() {
            return 2
        } else {
            return 0
        }
    }

    let n = s.len();
    let height = {
        let mut height = vec![0; n];
        let mut h = 0;
        for (i, c) in s.chars().enumerate() {
            match c {
                '(' => h += 1,
                ')' => h -= 1,
                _ => unreachable!()
            }
            height[i] = h;
        }
        height.insert(0, 0);
        height
    };

    let mut t: Vec<usize> = vec![0; n+1];
    t[n] = n;
    let mut longest = 0;
    for i in (0..n).rev() {
        if height[i] > height[i+1] {
            t[i] = i;
        } else {
            let mut p = i;
            loop {
                if p == n || height[p] > height[p+1] {
                    t[i] = p;
                    break;
                }
                let q = t[p+1];
                if q == n {
                    t[i] = p;
                    break;
                } else {
                    if height[q] > height[q+1] {
                        p = q+1;
                        continue
                    } else {
                        t[i] = p;
                        break;
                    }
                }
            }
        }
        longest = max(longest, t[i]-i);
    }

    longest as i32
}

#[test]
fn test_longest_valid_parentheses() {
    assert_eq!(longest_valid_parentheses("(()".to_string()), 2);
    assert_eq!(longest_valid_parentheses(")()())".to_string()), 4);
    assert_eq!(longest_valid_parentheses("()()((()".to_string()), 4);
    assert_eq!(longest_valid_parentheses(")(((((()())()()))()(()))(".to_string()), 22);
}