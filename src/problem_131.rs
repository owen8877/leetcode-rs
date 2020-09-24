pub fn partition(s: String) -> Vec<Vec<String>> {
    let s: Vec<char> = s.chars().rev().collect();
    let n = s.len();
    let mut cache = vec![vec![]; n+1];
    cache[n] = vec![vec![]];

    fn is_palindrome(s: &[char]) -> bool {
        let n = s.len();
        for i in 0..n/2 {
            if s[i] != s[n-1-i] {
                return false
            }
        }
        true
    }

    for i in (0..=n).rev() {
        for j in (i+1..=n) {
            if is_palindrome(&s[i..j]) {
                for mut v in cache[j].clone() {
                    v.push(s[i..j].iter().collect());
                    cache[i].push(v);
                }
            }
        }
    }

    cache[0].clone()
}