pub fn is_anagram(s: String, t: String) -> bool {
    let mut count = vec![0; 26];
    for c in s.chars() {
        count[(c as u8 - 'a' as u8) as usize] += 1;
    }
    for c in t.chars() {
        let index = (c as u8 - 'a' as u8) as usize;
        if count[index] == 0 {
            return false;
        }
        count[index] -= 1;
    }
    true
}