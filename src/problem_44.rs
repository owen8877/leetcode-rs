pub fn is_match_dfs(s: String, p: String) -> bool {
    use Wildcard::*;
    let empty: String = String::from("");

    if p == empty {
        return s == empty;
    }

    #[derive(Debug, PartialEq)]
    enum Wildcard {
        Single(char),
        AnySingle,
        AnyMultiple,
    }
    let mut wildcards = vec![];
    for c in p.chars() {
        match c {
            '*' => {
                if wildcards.is_empty() || wildcards.last().unwrap() != &AnyMultiple {
                    wildcards.push(AnyMultiple);
                }
            },
            '?' => wildcards.push(AnySingle),
            _ => wildcards.push(Single(c)),
        }
    }
    let mut estimate = vec![0; wildcards.len()];
    for i in (0..wildcards.len()-1).rev() {
        estimate[i] = estimate[i+1] + if wildcards[i+1] == AnyMultiple { 0 } else { 1 };
    }

    fn match_core(s: &[char], wildcards: &[Wildcard], estimate: &[usize], depth: usize) -> bool {
        // println!("-> {}", depth);
        let first_wildcard = wildcards.first();
        return match first_wildcard {
            None => s.len() == 0,
            Some(Single(c)) => {
                let first_char = s.first();
                match first_char {
                    Some(d) => c == d && match_core(&s[1..], &wildcards[1..], &estimate[1..], depth+1),
                    _ => false,
                }
            },
            Some(AnySingle) => s.len() > 0 && match_core(&s[1..], &wildcards[1..], &estimate[1..], depth+1),
            Some(AnyMultiple) => {
                let first_char = s.first();
                match first_char {
                    None => match_core(s, &wildcards[1..], &estimate[1..], depth+1),
                    _ => {
                        for i in 0..=(s.len()-estimate[0]) {
                            if match_core(&s[i..], &wildcards[1..], &estimate[1..], depth+1) {
                                return true
                            }
                        }
                        false
                    }
                }
            }
        }
    }

    println!("{:?}", wildcards);
    println!("{:?}", estimate);
    match_core(&s.chars().collect::<Vec<char>>()[..], &wildcards, &estimate, 0)
}

pub fn is_match_dp(s: String, p: String) -> bool {
    use Wildcard::*;
    let empty: String = String::from("");

    if p == empty {
        return s == empty;
    }
    if s == empty {
        return p.chars().fold(true, |acc, c | acc && (c == '*'))
    }

    #[derive(Debug, PartialEq)]
    enum Wildcard {
        Single(char),
        AnySingle,
        AnyMultiple,
    }
    let mut wildcards = vec![];
    for c in p.chars() {
        match c {
            '*' => {
                if wildcards.is_empty() || wildcards.last().unwrap() != &AnyMultiple {
                    wildcards.push(AnyMultiple);
                }
            },
            '?' => wildcards.push(AnySingle),
            _ => wildcards.push(Single(c)),
        }
    }
    let wildcards = wildcards;
    let target: Vec<char> = s.chars().collect();

    let m = target.len();
    let n = wildcards.len();
    let mut is_match_table = vec![vec![None; n+1]; m+1];

    // Init for boundary cases
    for i in 0..m {
        is_match_table[i][n] = Some(false);
    }
    is_match_table[m][n] = Some(true);
    let mut can_match_empty = true;
    for j in (0..n).rev() {
        if wildcards[j] != AnyMultiple {
            can_match_empty = false;
        }
        is_match_table[m][j] = Some(can_match_empty);
    }

    // Start matching
    fn core(is_match_table: &mut Vec<Vec<Option<bool>>>, target: &[char], wildcards: &[Wildcard], i: usize, j: usize) -> bool {
        if let Some(b) = is_match_table[i][j] {
            return b
        }
        let result = match wildcards[j] {
            Single(c) => target[i] == c && core(is_match_table, target, wildcards, i+1, j+1),
            AnySingle => core(is_match_table, target, wildcards, i+1, j+1),
            AnyMultiple => core(is_match_table, target, wildcards, i, j+1) || core(is_match_table, target, wildcards, i+1, j),
        };
        is_match_table[i][j] = Some(result);
        result
    }
    core(&mut is_match_table, target.as_slice(), wildcards.as_slice(), 0, 0)
}

#[test]
fn test_is_match() {
    let a = |s: &str, p: &str, e: bool| assert_eq!(is_match_dp(String::from(s), String::from(p)), e);

    a("aa", "a", false);
    a("aa", "*", true);
    a("cb", "?a", false);
    a("adceb", "*a*b", true);
    a("acdcb", "a*c?b", false);
    a("", "?", false);
    a("", "*", true);
    a("", "**", true);
    a("", "**a*", false);
    a("asdf", "", false);
    a("aaaabaaaabbbbaabbbaabbaababbabbaaaababaaabbbbbbaabbabababbaaabaabaaaaaabbaabbbbaababbababaabbbaababbbba", "*****b*aba***babaa*bbaba***a*aaba*b*aa**a*b**ba***a*a*", true);
    a("abbabaaabbabbaababbabbbbbabbbabbbabaaaaababababbbabababaabbababaabbbbbbaaaabababbbaabbbbaabbbbababababbaabbaababaabbbababababbbbaaabbbbbabaaaabbababbbbaababaabbababbbbbababbbabaaaaaaaabbbbbaabaaababaaaabb", "**aa*****ba*a*bb**aa*ab****a*aaaaaa***a*aaaa**bbabb*b*b**aaaaaaaaa*a********ba*bbb***a*ba*bb*bb**a*b*bb", false);
}