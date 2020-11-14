pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
    use std::collections::*;

    let mut word_map = HashMap::new();
    let mut blank_position = None;

    for (i, word) in words.iter().enumerate() {
        word_map.insert(word.as_str(), i);
        if word.as_str() == "" {
            blank_position = Some(i);
        }
    }

    let mut result = vec![];

    fn is_palindrome(s: &str) -> bool {
        s.chars().eq(s.chars().rev())
    }

    for (i, word) in words.iter().enumerate() {
        if word.len() < 1 {
            continue;
        }

        if word.len() == 1 {
            if let Some(j) = blank_position {
                result.push(vec![i as i32, j as i32]);
                result.push(vec![j as i32, i as i32]);
            }
            continue;
        }

        if !is_palindrome(word) {
            let reverse = word.chars().rev().collect::<String>();
            if let Some(&l) = word_map.get(&reverse.as_str()) {
                result.push(vec![l as i32, i as i32]);
            }
        } else {
            if let Some(j) = blank_position {
                result.push(vec![i as i32, j as i32]);
                result.push(vec![j as i32, i as i32]);
            }
        }

        for j in 1..word.len() {
            let first = &word[..j];
            let second = &word[j..];

            if is_palindrome(first) {
                let second_reverse = second.chars().rev().collect::<String>();
                if let Some(&l) = word_map.get(&second_reverse.as_str()) {
                    result.push(vec![l as i32, i as i32]);
                }
            }

            if is_palindrome(second) {
                let first_reverse = first.chars().rev().collect::<String>();
                if let Some(&l) = word_map.get(&first_reverse.as_str()) {
                    result.push(vec![i as i32, l as i32]);
                }
            }
        }
    }

    result
}