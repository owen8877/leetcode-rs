pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let s = s.chars().collect::<Vec<char>>();
    let ns = s.len();
    let word_dict: Vec<Vec<char>> = word_dict.into_iter().map(|s| s.chars().collect::<Vec<char>>()).collect();
    let nw = word_dict.len();
    let mut cache = vec![false; ns+1];
    cache[ns] = true;

    fn partial_match(a: &[char], b: &[char]) -> bool {
        if a.len() < b.len() {
            false
        } else {
            for i in 0..b.len() {
                if a[i] != b[i] {
                    return false
                }
            }
            true
        }
    }

    for i in (0..ns).rev() {
        for word in word_dict.iter() {
            if partial_match(&s[i..], word.as_slice()) && cache[i+word.len()] {
                cache[i] = true;
                break
            }
        }
    }

    cache[0]
}