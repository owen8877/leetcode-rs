pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    if k == 0 {
        return vec![vec![]]
    } else if k == 1 {
        return (1..=n).map(|x| vec![x]).collect()
    } else if n == k {
        return vec![(1..=n).collect()]
    } else if n < k {
        return vec![vec![]]
    }

    let mut result = combine(n-1, k);
    for mut v in combine(n-1, k-1) {
        v.push(n);
        result.push(v);
    }
    result
}

#[test]
fn test_combine() {
    assert_eq!(combine(4, 2).len(), 6);
}