pub fn is_subsequence(s: String, t: String) -> bool {
    fn core(s: &str, t: &str) -> bool {
        if s.len() == 0 {
            return true
        }
        if t.len() == 0 {
            return false
        }
        if s.len() > t.len() {
            return false
        }
        // Now we can safely assume 1 <= s.len() <= t.len()
        let s1 = s.chars().next().unwrap();
        let t1 = t.chars().next().unwrap();
        if s1 == t1 {
            core(&s[1..], &t[1..])
        } else {
            core(s, &t[1..])
        }
    }
    core(s.as_str(), t.as_str())
}

#[test]
fn test_is_subsequence() {
    assert_eq!(is_subsequence("abc".to_string(), "ahbgdc".to_string()), true);
    assert_eq!(is_subsequence("axc".to_string(), "ahbgdc".to_string()), false);
}