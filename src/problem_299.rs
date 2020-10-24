pub fn get_hint(secret: String, guess: String) -> String {
    use std::collections::*;
    let mut same = 0;
    let mut different_1 = HashMap::new();
    let mut different_2 = HashMap::new();
    for (i, j) in secret.chars().zip(guess.chars()) {
        let i = i as u8 - '0' as u8;
        let j = j as u8 - '0' as u8;
        if i == j {
            same += 1;
        } else {
            *different_1.entry(i).or_insert(0) += 1;
            *different_2.entry(j).or_insert(0) += 1;
        }
    }
    let mut different = 0;
    for j in 0..10 {
        different += std::cmp::min(different_1.get(&j).unwrap_or(&0), different_2.get(&j).unwrap_or(&0));
    }
    format!("{}A{}B", same, different)
}