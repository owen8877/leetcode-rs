pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.len() == 0 {
        return vec![]
    }

    fn core(s: &str) -> Vec<String> {
        let arr = vec![
            vec![" "],
            vec![""],
            vec!["a", "b", "c"],
            vec!["d", "e", "f"],
            vec!["g", "h", "i"],
            vec!["j", "k", "l"],
            vec!["m", "n", "o"],
            vec!["p", "q", "r", "s"],
            vec!["t", "u", "v"],
            vec!["w", "x", "y", "z"],
        ];

        let first = s.chars().next().unwrap();
        let letters = arr[first as usize - '0' as usize].clone();
        if s.len() == 1 {
            return letters.iter().map(|x| x.to_string()).collect()
        }
        let comb = core(&s[1..]);
        if first == '1' {
            comb
        } else {
            let mut result = Vec::<String>::new();
            for prefix in letters {
                for c in &comb {
                    result.push(prefix.to_string() + c.clone().as_str());
                }
            }
            result
        }
    }

    core(digits.as_str())
}

#[test]
fn test_letter_combinations() {
    println!("{:?}", letter_combinations("23".to_string()));
    println!("{:?}", letter_combinations("123".to_string()));
}