pub fn min_cut(s: String) -> i32 {
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    if n < 2 {
        return 0
    }

    fn is_palindrome(s: &[char]) -> bool {
        let n = s.len();
        for i in 0..n/2 {
            if s[i] != s[n-1-i] {
                return false
            }
        }
        true
    }

    let mut min_cut = vec![0; n];
    for i in (0..n).rev() {
        if is_palindrome(&s[i..n]) {
            continue
        }
        let mut m = n-i;
        for j in i+1..n {
            if m <= min_cut[j] {
                continue
            }
            if is_palindrome(&s[i..j]) {
                m = std::cmp::min(m, min_cut[j]);
            }
        }
        min_cut[i] = m + 1;
    }
    min_cut[0] as i32
}