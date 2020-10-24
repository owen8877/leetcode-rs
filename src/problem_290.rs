pub fn word_pattern(pattern: String, s: String) -> bool {
    use std::collections::*;
    if pattern.len() != s.split(' ').count() {
        return false;
    }
    let mut map = HashMap::new();
    for (c, word) in pattern.chars().zip(s.split(' ')) {
        match map.get(&c).clone() {
            None => {
                map.insert(c, word);
            }
            Some(w2) => {
                if &word != w2 {
                    return false;
                }
            }
        }
    }
    let mut set = HashSet::new();
    for word in map.values() {
        set.insert(word);
    }
    set.len() == map.len()
}