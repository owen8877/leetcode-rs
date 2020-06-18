pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.len() == 0 {
         return 0
    }
    if haystack.len() == 0 {
        return -1
    }

    let n = needle.len();
    let h_vec: Vec<char> = haystack.chars().collect();
    let n_vec: Vec<char> = needle.chars().collect();
    let mut failure_table: Vec<i32> = vec![-1; n];
    for i in 1..n {
        if i == 1 {
            failure_table[i] = 0;
            continue
        }
        if i == 2 {
            if n_vec[0] == n_vec[1] {
                failure_table[i] = 1;
            } else {
                failure_table[i] = 0;
            }
            continue
        }
        let mut lsp = failure_table[i-1];
        while lsp >= 0 {
            if n_vec[lsp as usize] == n_vec[i-1] {
                break;
            } else {
                lsp = failure_table[lsp as usize];
            }
        }
        failure_table[i] = lsp + 1;
    }

    let mut m = 0;
    let mut i = 0;

    while i+m < haystack.len() {
        if i == n {
            return m as i32
        }
        if h_vec[i+m] == n_vec[i] {
            i += 1;
            if i == n {
                return m as i32
            }
            continue
        } else {
            match failure_table[i] {
                -1 => m += 1,
                n => {
                    m += i - n as usize;
                    i = n as usize;
                },
            }
        }
    }

    -1
}

#[test]
fn test_str_str() {
    assert_eq!(str_str("aabaaabaaac".to_string(), "aabaaac".to_string()), 4);
    assert_eq!(str_str("a".to_string(), "a".to_string()), 0);
    assert_eq!(str_str("aaa".to_string(), "aaaa".to_string()), -1);
    assert_eq!(str_str("hello".to_string(), "abcababcd".to_string()), -1);
    assert_eq!(str_str("hello".to_string(), "ll".to_string()), 2);
    assert_eq!(str_str("aaaaaa".to_string(), "bba".to_string()), -1);
    assert_eq!(str_str("aaaaaa".to_string(), "".to_string()), 0);
}