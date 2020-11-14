pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    use std::cmp::*;
    let n = matrix.len();
    if n == 0 {
        return 0;
    }
    let m = matrix[0].len();
    if m == 0 {
        return 0;
    }
    let mut table = vec![vec![None; m]; n];

    fn core(i: usize, j: usize, n: usize, m: usize, matrix: &Vec<Vec<i32>>, table: &mut Vec<Vec<Option<i32>>>) -> i32 {
        if let Some(l) = table[i][j] {
            return l;
        }

        let mut l = 1;
        if i > 0 && matrix[i - 1][j] > matrix[i][j] {
            l = max(l, core(i - 1, j, n, m, matrix, table) + 1);
        }
        if j > 0 && matrix[i][j - 1] > matrix[i][j] {
            l = max(l, core(i, j - 1, n, m, matrix, table) + 1);
        }
        if i + 1 < n && matrix[i + 1][j] > matrix[i][j] {
            l = max(l, core(i + 1, j, n, m, matrix, table) + 1);
        }
        if j + 1 < m && matrix[i][j + 1] > matrix[i][j] {
            l = max(l, core(i, j + 1, n, m, matrix, table) + 1);
        }
        table[i][j] = Some(l);
        l
    }

    let mut longest = 0;
    for i in 0..n {
        for j in 0..m {
            longest = max(longest, core(i, j, n, m, &matrix, &mut table));
        }
    }
    longest
}