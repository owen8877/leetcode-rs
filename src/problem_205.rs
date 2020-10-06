pub fn is_isomorphic(s: String, t: String) -> bool {
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();

    use std::collections::*;
    let mut map = HashMap::new();
    let mut set = HashSet::new();

    for (&cs, &ct) in s.iter().zip(t.iter()) {
        if let Some(&ct_old) = map.get(&cs) {
            if ct_old != ct {
                return false
            }
        } else {
            map.insert(cs, ct);
            set.insert(ct);
            if map.len() != set.len() {
                return false
            }
        }
    }
    true
}