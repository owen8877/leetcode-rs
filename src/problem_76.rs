pub fn min_window(s: String, t: String) -> String {
    use std::collections::HashMap;
    if s.len() == 0 || t.len() == 0 {
        return "".to_string()
    }

    let m = s.len();
    let n = t.len();
    let s_vec: Vec<char> = s.chars().collect();
    let t_vec: Vec<char> = t.chars().collect();

    let mut left = 0;
    let mut right = 0;
    let mut lrl_pair: Option<(usize, usize, usize)> = None;
    let mut desirable = false;
    let mut cumulative = HashMap::<char, usize>::new();
    let mut requirement = HashMap::<char, usize>::new();
    for t_c in t_vec.iter() {
        requirement.entry(*t_c).and_modify(|x| *x += 1).or_insert(1);
        cumulative.entry(*t_c).or_insert(0);
    }
    if cumulative.contains_key(&s_vec[0]) {
        cumulative.insert(s_vec[0], 1);
        desirable = cumulative.iter().fold(true, |acc, x| acc && x.1 >= requirement.get(x.0).unwrap());
    }
    while right < m {
        if desirable {
            let c = s_vec[left];
            if let Some(number) = cumulative.get(&c) {
                if number <= requirement.get(&c).unwrap() {
                    let len = right - left + 1;
                    if lrl_pair.is_none() || len < lrl_pair.unwrap().2 {
                        lrl_pair = Some((left, right, len));
                    }
                    desirable = false;
                }
                cumulative.insert(c, number-1);
            }
            left += 1;
        } else {
            right += 1;
            if right >= m {
                break
            }
            let c = s_vec[right];
            if let Some(k) = cumulative.get(&c) {
                cumulative.insert(c, k+1);
                desirable = cumulative.iter().fold(true, |acc, x| acc && x.1 >= requirement.get(x.0).unwrap());
            }
        }
    }

    if let Some((left, right, _)) = lrl_pair {
        s_vec[left..=right].iter().collect()
    } else {
        "".to_string()
    }
}

#[test]
fn test_min_window() {
    assert_eq!(min_window("ADOBECODEBANC".to_string(), "ABC".to_string()), "BANC".to_string());
    assert_eq!(min_window("a".to_string(), "aa".to_string()), "".to_string());
}