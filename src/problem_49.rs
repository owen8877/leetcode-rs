// Need improvement
// Reference: https://leetcode.com/problems/group-anagrams/discuss/264561/A-Rust-impl-with-8ms
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;

    let mut map = HashMap::<Vec<char>, Vec<String>>::new();
    for s in strs {
        let mut vc: Vec<char> = s.chars().collect();
        vc.sort();
        match map.get_mut(&vc) {
            None => {
                map.insert(vc, vec![s]);
            },
            Some(v) => {
                v.push(s);
            },
        }
    }

    map.values().map(|x| x.clone()).collect()
}