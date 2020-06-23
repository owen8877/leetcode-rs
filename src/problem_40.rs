// Need rework
pub fn combination_sum2_hashset(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    use std::collections::HashSet;
    let mut candidates = candidates;
    candidates.sort_by(|a, b| (-a).cmp(&(-b)));

    fn core(c: &[i32], target: i32, prefix: Vec<i32>) -> Vec<Vec<i32>> {
        if target == 0 {
            return vec![prefix]
        }

        if c.len() == 1 {
            let n = c[0];
            if target != n {
                return vec![]
            }
            let mut p = prefix.clone();
            for _ in 0..target/n {
                p.push(n)
            }
            return vec![p]
        }

        let mut result = HashSet::<Vec<i32>>::new();
        let n = c[0];
        if target < n {
            for v in core(&c[1..], target, prefix) {
                result.insert(v);
            }
        } else {
            for v in core(&c[1..], target, prefix.clone()) {
                result.insert(v);
            }
            let mut p = prefix.clone();
            p.push(n);
            for v in core(&c[1..], target-n, p) {
                result.insert(v);
            }
        }

        result.into_iter().collect()
    }

    core(&candidates, target, vec![])
}

pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort_by(|a, b| (-a).cmp(&(-b)));

    fn core(c: &[i32], target: i32, prefix: Vec<i32>, forbidden: Option<i32>) -> Vec<Vec<i32>> {
        if target == 0 {
            return vec![prefix]
        }

        if c.len() == 1 {
            let n = c[0];
            if target != n {
                return vec![]
            }
            if let Some(f) = forbidden {
                if f == n {
                    return vec![]
                }
            }
            let mut p = prefix.clone();
            p.push(n);
            return vec![p]
        }

        let mut result = Vec::<Vec<i32>>::new();
        let n = c[0];
        if target < n {
            for v in core(&c[1..], target, prefix, forbidden) {
                result.push(v);
            }
        } else {
            let get_forbidden = {
                if let Some(f) = forbidden {
                    f == n
                } else { false }
            };
            if !get_forbidden {
                let mut p = prefix.clone();
                p.push(n);
                for v in core(&c[1..], target-n, p, None) {
                    result.push(v);
                }
            }

            for v in core(&c[1..], target, prefix, Some(n)) {
                result.push(v);
            }
        }

        result.into_iter().collect()
    }

    core(&candidates, target, vec![], None)
}

#[test]
fn test_combination_sum() {
    assert_eq!(combination_sum2(vec![10,1,2,7,6,1,5], 8),
               vec![vec![1, 7], vec![1, 2, 5], vec![2, 6], vec![1, 1, 6]]);
    assert_eq!(combination_sum2(vec![2,5,2,1,2], 5), vec![vec![5], vec![1, 2, 2]]);
}