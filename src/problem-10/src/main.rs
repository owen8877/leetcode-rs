pub fn is_match(s: String, p: String) -> bool {
    let empty: String = String::from("");

    if p == empty {
        return s == empty;
    }

    #[derive(Debug)]
    enum Reg {
        Single(char),
        Multiple(char),
        AnySingle,
        AnyMultiple,
    }
    let mut regs: Vec<Reg> = Vec::<Reg>::new();
    if p.starts_with("*") {
        panic!("Cannot understand regex {}.", p);
    }
    let mut star_on = false;
    for current_char in p.chars().rev() {
        match current_char {
            '.' => {
                if star_on {
                    regs.push(Reg::AnyMultiple);
                } else {
                    regs.push(Reg::AnySingle);
                }
                star_on = false;
            },
            '*' => {
                star_on = true;
            },
            _ => {
                if star_on {
                    regs.push(Reg::Multiple(current_char));
                } else {
                    regs.push(Reg::Single(current_char));
                }
                star_on = false;
            }
        }
    }
    regs.reverse();

    fn match_core(s: &[char], regs: &[Reg]) -> bool {
        let first_reg = regs.first();
        return match first_reg {
            None => s.len() == 0,
            Some(Reg::Single(c)) => {
                let first_char = s.first();
                match first_char {
                    Some(d) => c == d && match_core(&s[1..], &regs[1..]),
                    _ => false,
                }
            },
            Some(Reg::Multiple(c)) => {
                let first_char = s.first();
                match first_char {
                    None => match_core(s, &regs[1..]),
                    Some(d) => {
                        if c == d {
                            let max_match_length = find_longest_consecutive(s);
                            for i in (0..=max_match_length).rev() {
                                if match_core(&s[i..], &regs[1..]) {
                                    return true;
                                }
                            }
                            false
                        } else {
                            match_core(s, &regs[1..])
                        }
                    }
                }
            },
            Some(Reg::AnySingle) => s.len() > 0 && match_core(&s[1..], &regs[1..]),
            Some(Reg::AnyMultiple) => {
                let first_char = s.first();
                match first_char {
                    None => match_core(s, &regs[1..]),
                    _ => {
                        for i in (0..=s.len()).rev() {
                            if match_core(&s[i..], &regs[1..]) {
                                return true;
                            }
                        }
                        false
                    }
                }
            }
        }
    }

    match_core(&s.chars().collect::<Vec<char>>()[..], &regs)
}

fn find_longest_consecutive(s: &[char]) -> usize {
    if s.len() == 0 { panic!("s should not be empty here!"); }

    let first_char = s.first().unwrap().clone();
    for i in 1..s.len() {
        if first_char != s[i] {
            return i;
        }
    }
    s.len()
}

#[test]
fn test_is_match() {
    let a = |s: &str, p: &str, e: bool| assert_eq!(is_match(String::from(s), String::from(p)), e);

    a("aa", "a", false);
    a("aa", "a*", true);
    a("aab", ".*", true);
    a("aab", "c*a*b", true);
    a("mississippi", "mis*is*p*.", false);
    a("", ".", false);
    a("", "", true);
    a("asdf", "", false);

    assert_eq!(find_longest_consecutive(&"ccccaab".chars().collect::<Vec<char>>()[..]), 4);
}