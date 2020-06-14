pub fn longest_common_prefix(strs: Vec<String>) -> String {
    use std::str::Chars;

    if strs.len() == 0 {
        return "".to_string()
    }

    if strs.len() == 1 {
        return strs[0].to_string()
    }

    let mut charsvec: Vec<Chars> = strs.iter().map(|str| str.chars()).collect();

    let mut longest = 0;
    loop {
        match charsvec[0].next() {
            None => {
                return String::from(&strs[0][0..longest])
            },
            Some(c) => {
                for i in 1..charsvec.len() {
                    match charsvec[i].next() {
                        None => {
                            return String::from(&strs[0][0..longest])
                        },
                        Some(d) => {
                            if c != d {
                                return String::from(&strs[0][0..longest])
                            }
                        }
                    }
                }
                longest += 1;
            },
        }
    }

    strs[0].to_string()
}

#[test]
fn test_longest_common_prefix() {
    assert_eq!(longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]), "fl".to_string());
    assert_eq!(longest_common_prefix(vec!["flower".to_string(), "flower".to_string(), "flower".to_string()]), "flower".to_string());
    assert_eq!(longest_common_prefix(vec!["animal".to_string(), "vcat".to_string(), "dog".to_string()]), "".to_string());
}