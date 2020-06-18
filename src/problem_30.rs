// TLE
pub fn find_substring_1(s: String, words: Vec<String>) -> Vec<i32> {
    use std::collections::HashSet;

    let n = words.len();
    let m = words[0].len();
    let ns = s.len();
    if n == 0  || m == 0 {
        return vec![]
    }
    if ns < n * m {
        return vec![]
    }

    fn perm(n: usize) -> Vec<Vec<usize>> {
        match n {
            0 => unreachable!(),
            1 => vec![vec![0]],
            2 => vec![vec![0, 1], vec![1, 0]],
            n => {
                let permutations = perm(n-1);
                let mut result = Vec::<Vec<usize>>::with_capacity(permutations.len() * n);
                for p in permutations {
                    for j in 0..n {
                        let mut r = Vec::<usize>::with_capacity(n);
                        for i in 0..j {
                            r.push(p[i]);
                        }
                        r.push(n-1);
                        for i in j..n-1 {
                            r.push(p[i]);
                        }
                        result.push(r);
                    }
                }
                result
            }
        }
    }

    let permutations = perm(n);
    let occurance = {
        let mut table = Vec::<HashSet<usize>>::new();

        for i in 0..n {
            table.push(HashSet::new());
            for j in 0..=ns-m {
                if &s.as_str()[j..j+m] == words[i].as_str() {
                    table[i].insert(j);
                }
            }
        }

        table
    };

    let mut result = HashSet::<i32>::new();
    for l in 0..=ns-n*m {
        'permut: for p in &permutations {
            for i in 0..n {
                if !occurance[p[i]].contains(&(l+i*m)) {
                    continue 'permut
                }
            }
            result.insert(l as i32);
        }
    }

    result.into_iter().collect()
}

pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    use std::collections::HashMap;

    if words.len() == 0  || words[0].len() == 0 {
        return vec![]
    }
    let n = words.len();
    let m = words[0].len();
    let ns = s.len();

    if ns < n * m {
        return vec![]
    }

    let occurance = {
        let mut o = HashMap::<String, i32>::new();
        for word in &words {
            match o.get(word) {
                None => {
                    o.insert(word.clone(), 1);
                },
                Some(n) => {
                    o.insert(word.clone(), n+1);
                },
            }
        }
        o
    };

    let mut result = vec![];
    'outer: for i in 0..=ns-n*m {
        let mut o = occurance.clone();
        for l in 0..n {
            let segment = &s.as_str()[i+l*m..i+(l+1)*m].to_string();
            match o.get(segment) {
                None => continue 'outer,
                Some(n) => {
                    if n == &0 {
                        continue 'outer
                    } else {
                        o.insert(segment.clone(), n-1);
                    };
                },
            }
        }
        result.push(i as i32);
    }
    result
}

#[test]
fn test_find_substring() {
    assert_eq!(find_substring("wordgoodgoodgoodbestword".to_string(), vec!["word".to_string(), "good".to_string(), "best".to_string(), "good".to_string()]), vec![8]);
    assert_eq!(find_substring("barfoothefoobarman".to_string(), vec!["foo".to_string(), "bar".to_string()]), vec![0, 9]);
    assert_eq!(find_substring("wordgoodgoodgoodbestword".to_string(), vec!["word".to_string(), "good".to_string(), "best".to_string(), "word".to_string()]), vec![]);
}