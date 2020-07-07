pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    use std::cmp::min;
    let m = grid.len();
    let n = grid[0].len();
    let mut min_sum = vec![vec![None; n]; m];
    min_sum[m-1][n-1] = Some(grid[m-1][n-1]);

    fn sum(min_sum: &mut Vec<Vec<Option<i32>>>, grid: &Vec<Vec<i32>>, i: usize, j: usize, m: usize, n: usize) {
        if min_sum[i][j].is_some() {
            return
        }
        let mut cost = 1 << 30;
        if i + 1 < m {
            if min_sum[i+1][j].is_none() {
                sum(min_sum, grid, i+1, j, m, n);
            }
            cost = min(min_sum[i+1][j].unwrap(), cost);
        }
        if j + 1 < n {
            if min_sum[i][j+1].is_none() {
                sum(min_sum, grid, i, j+1, m, n);
            }
            cost = min(min_sum[i][j+1].unwrap(), cost);
        }
        min_sum[i][j] = Some(cost+grid[i][j]);
    }

    sum(&mut min_sum, &grid, 0, 0, m, n);
    min_sum[0][0].unwrap()
}