pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    use std::cmp::*;
    use std::collections::*;

    fn add(v1: &mut Vec<i32>, v2: &Vec<i32>) {
        for i in 0..v1.len() {
            v1[i] += v2[i];
        }
    }

    fn max_sum_under(k: i32, arr: &[i32]) -> Option<i32> {
        let mut set = BTreeSet::new();
        set.insert(0);
        let mut prefix = 0;
        let mut max_sum = None;
        for &num in arr {
            prefix += num;

            let range = set.range(prefix - k..);
            if let Some(m) = set.range(prefix - k..).next() {
                let sum = prefix - m;
                max_sum = Some(max_sum.map_or(sum, |mm| max(mm, sum)));
            }

            set.insert(prefix);
        }
        max_sum
    }

    let mut max_sum = None;
    for i in 0..matrix.len() {
        let mut v = vec![0; matrix[0].len()];

        for j in i..matrix.len() {
            add(&mut v, &matrix[j]);
            if let Some(m) = max_sum_under(k, &v) {
                max_sum = Some(max_sum.map_or(m, |mm| max(m, mm)));
            }
        }
    }

    max_sum.unwrap()
}