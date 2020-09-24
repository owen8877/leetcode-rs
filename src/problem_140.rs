pub fn word_break(s: String, words: Vec<String>) -> Vec<String> {
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    let word_dict: Vec<Vec<char>> = words.iter().map(|s| s.chars().collect::<Vec<char>>()).collect();
    let mut partial_match: Vec<Vec<Vec<usize>>> = vec![vec![]; n+1];
    let mut can_partial_match: Vec<bool> = vec![false; n+1];
    partial_match[n] = vec![vec![]];
    can_partial_match[n] = true;
    let max_length = words.iter().fold(0, |acc, x| std::cmp::max(acc, x.len()));

    for j in (0..n).rev() {
        for (i, word) in word_dict.iter().enumerate() {
            let end_index = j + word.len();
            if n >= end_index && &s[j..end_index] == word.as_slice() && can_partial_match[end_index] {
                can_partial_match[j] = true;
                break
            }
        }
    }

    if !can_partial_match[0] {
        return vec![]
    }

    for j in (0..n).rev() {
        let free_index = j+max_length+1;
        if free_index < n {
            partial_match[free_index] = vec![];
        }
        for (i, word) in word_dict.iter().enumerate() {
            let end_index = j + word.len();
            if n >= end_index && &s[j..end_index] == word.as_slice() {
                let mut pm = partial_match[end_index].clone().into_iter().map(|mut v| {
                    v.insert(0, i);
                    v
                }).collect();
                partial_match[j].append(&mut pm);
            }
        }
    }

    partial_match[0].iter().map(|v| v.iter().enumerate().fold("".to_string(), |acc, (i, &x)| {
        if i == 0 {
            words[x].clone()
        } else {
            acc + " " + words[x].as_str()
        }
    })).collect()
}