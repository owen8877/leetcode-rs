pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    if s.len() < 11 {
        return vec![]
    }
    let mut map = std::collections::HashMap::new();
    let s: Vec<char> = s.chars().collect();
    for i in 0..s.len()-9 {
        *map.entry(&s[i..i+10]).or_insert(0) += 1;
    }
    map.into_iter().filter(|(_, i)| i>&1).map(|(s, _)| s.iter().collect()).collect()
}