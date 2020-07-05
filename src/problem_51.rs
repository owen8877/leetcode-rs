pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    if n < 4 {
        if n == 1 {
            return vec![vec!["Q".to_string()]]
        }
        return vec![]
    }
    let n = n as usize;
    let mut result = vec![];

    fn core(index: usize, v: &mut Vec<usize>, r: &mut Vec<Vec<usize>>, push_two: bool) {
        let n = v.len();
        if index == n {
            r.push(v.clone());
            if push_two {
                r.push(v.into_iter().map(|x| n-1-*x).collect());
            }
            return
        }
        'outer: for i in 0..n {
            for j in 0..index {
                if v[j] == i || v[j] + j == i + index || v[j] + index == i + j {
                    continue 'outer
                }
            }
            v[index] = i;
            core(index+1, v, r, push_two)
        }
    }

    for i in 0..n/2 {
        let mut v = vec![0; n];
        v[0] = i;
        core(1, &mut v, &mut result, true);
    }
    if n % 2  == 1 {
        let mut v = vec![0; n];
        v[0] = n/2;
        core(1, &mut v, &mut result, false);
    }

    result.iter().map(|v| {
        let template = vec!['.'; n];
        let mut r = vec![];
        for i in 0..n {
            let mut tmp = template.clone();
            tmp[v[i]] = 'Q';
            r.push(tmp.iter().collect())
        }
        r
    }).collect()
}

#[test]
fn test_solve_n_queens() {
    assert_eq!(solve_n_queens(4).len(), 2);
    assert_eq!(solve_n_queens(8).len(), 92);
    assert_eq!(solve_n_queens(9).len(), 352);
}
