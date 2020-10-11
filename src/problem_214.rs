pub fn shortest_palindrome(s: String) -> String {
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    if n < 2 {
        return s.iter().map(|&c| c).collect()
    }
    let mut end_i = n;
    loop {
        let mut i = 0;
        for j in (0..end_i).rev() {
            if s[i] == s[j] {
                i += 1;
            }
        }
        if end_i == i {
            break
        } else {
            end_i = i;
        }
    }
    let s_rev: String = (s[end_i..].iter().map(|&c| c)).rev().collect();
    s_rev + s.iter().map(|&c| c).collect::<String>().as_str()
}