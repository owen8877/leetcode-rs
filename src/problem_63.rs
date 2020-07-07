pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();
    let mut valid_ways = vec![vec![None; n]; m];
    valid_ways[m-1][n-1] = Some(1 - obstacle_grid[m-1][n-1]);

    fn count_ways(valid_ways: &mut Vec<Vec<Option<i32>>>, obstacle_grid: &Vec<Vec<i32>>, i: usize, j: usize, m: usize, n: usize) {
        if obstacle_grid[i][j] == 1 {
            valid_ways[i][j] = Some(0);
            return
        }
        if valid_ways[i][j].is_some() {
            return
        }
        let mut sum = 0;
        if i + 1 < m && obstacle_grid[i+1][j] == 0 {
            if valid_ways[i+1][j].is_none() {
                count_ways(valid_ways, obstacle_grid, i+1, j, m, n);
            }
            sum += valid_ways[i+1][j].unwrap();
        }
        if j + 1 < n && obstacle_grid[i][j+1] == 0 {
            if valid_ways[i][j+1].is_none() {
                count_ways(valid_ways, obstacle_grid, i, j+1, m, n);
            }
            sum += valid_ways[i][j+1].unwrap();
        }
        valid_ways[i][j] = Some(sum);
    }

    count_ways(&mut valid_ways, &obstacle_grid, 0, 0, m, n);
    valid_ways[0][0].unwrap()
}

#[test]
fn test_unique_paths_with_obstacles() {
    assert_eq!(unique_paths_with_obstacles(vec![vec![0]]), 1);
}