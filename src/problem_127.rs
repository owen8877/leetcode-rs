// TODO: Needs improvement
pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    use std::collections::HashSet;

    let begin_word: Vec<char> = begin_word.chars().collect();
    let end_word: Vec<char> = end_word.chars().collect();
    let word_list: Vec<Vec<char>> = word_list.into_iter().map(|s| s.chars().collect::<Vec<char>>()).collect();
    let n = word_list.len();

    let mut neighbours = vec![HashSet::new(); n];

    fn diff_once(a: &[char], b: &[char]) -> bool {
        let mut mismatch = 0;
        for (&aa, &bb) in a.iter().zip(b.iter()) {
            if aa != bb {
                mismatch += 1;
            }
            if mismatch >= 2 {
                return false
            }
        }
        true
    }

    fn same(a: &[char], b: &[char]) -> bool {
        for (&aa, &bb) in a.iter().zip(b.iter()) {
            if aa != bb {
                return false
            }
        }
        true
    }

    for i in 0..n {
        for j in 0..i {
            if diff_once(word_list[i].as_slice(), word_list[j].as_slice()) {
                neighbours[i].insert(j);
                neighbours[j].insert(i);
            }
        }
    }

    let mut start_set = HashSet::new();
    for i in 0..n {
        if diff_once(word_list[i].as_slice(), begin_word.as_slice()) {
            start_set.insert(i);
        }
    }
    let mut active_set = HashSet::new();
    let mut visited_set = HashSet::new();
    for i in 0..n {
        if same(word_list[i].as_slice(), end_word.as_slice()) {
            active_set.insert(i);
        }
    }

    let mut steps = 2;
    while !active_set.is_empty() {
        for &j in active_set.iter() {
            if start_set.contains(&j) {
                return steps
            }
        }
        let mut newly_added = HashSet::new();
        for &j in active_set.iter() {
            for &k in neighbours[j].iter() {
                if !visited_set.contains(&k) && !active_set.contains(&k) {
                    newly_added.insert(k);
                }
            }
        }
        for &j in active_set.iter() {
            visited_set.insert(j);
        }
        active_set = newly_added;
        steps += 1;
    }
    0
}