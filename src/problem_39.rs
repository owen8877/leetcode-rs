pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort_by(|a, b| (-a).cmp(&(-b)));

    fn core(c: &[i32], target: i32, prefix: Vec<i32>) -> Vec<Vec<i32>> {
        if c.len() == 1 {
            let n = c[0];
            if target % n != 0 {
                return vec![]
            }
            let mut p = prefix.clone();
            for _ in 0..target/n {
                p.push(n)
            }
            return vec![p]
        }

        let mut result = vec![];
        let n = c[0];
        for i in 0..=target/n {
            let mut p = prefix.clone();
            for _ in 0..i {
                p.push(n);
            }
            for v in core(&c[1..], target-i*n, p) {
                result.push(v)
            }
        }

        result
    }

    core(&candidates, target, vec![])
}

#[test]
fn test_combination_sum() {
    assert_eq!(combination_sum(vec![2,3,6,7], 7), vec![vec![3, 2, 2], vec![7]]);
    assert_eq!(combination_sum(vec![2,3,5], 8), vec![vec![5, 3], vec![3, 3, 2], vec![2, 2, 2, 2]]);
}