pub fn longest_palindrome(s: String) -> String {
    use std::cmp::min;
    let v: Vec<char> = s.chars().collect();
    let n = v.len();

    if n <= 1 {
        return s;
    }

    let mut longest = 1;
    let mut start = 0;
    let mut end = 1;

    // Odd
    for i in 1..n-1 {
        let mut palindrome = true;
        for j in 1..min(i, n-i-1)+1 {
            if v[i + j] != v[i - j] {
                let length = 2*j-1;
                if length > longest {
                    longest = length;
                    start = i-j+1;
                    end = i+j;
                }
                palindrome = false;
                break;
            }
        }
        if palindrome {
            let j = min(i, n-i-1);
            let length = 2*j+1;
            if length > longest {
                longest = length;
                start = i-j;
                end = i+j+1;
            }
        }
    }

    // Even
    for i in 0..n-1 {
        let mut palindrome = true;
        for j in 0..min(i, n-i-2)+1 {
            if v[i + 1 + j] != v[i - j] {
                let length = 2*j;
                if length > longest {
                    longest = length;
                    start = i-j+1;
                    end = i+j+1;
                }
                palindrome = false;
                break;
            }
        }
        if palindrome {
            let j = min(i, n-i-2);
            let length = 2*j+2;
            if length > longest {
                longest = length;
                start = i-j;
                end = i+j+2;
            }
        }
    }

    s.chars().skip(start).take(end-start).collect::<String>()
}

#[test]
fn test_longest_palindrome() {
    assert_eq!(longest_palindrome(String::from("ac")), "a");
    assert_eq!(longest_palindrome(String::from("aa")), "aa");
    assert_eq!(longest_palindrome(String::from("babad")), "bab");
    assert_eq!(longest_palindrome(String::from("babbab")), "babbab");
}