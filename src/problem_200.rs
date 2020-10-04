pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let n = grid.len();
    if n == 0 {
        return 0
    }

    let m = grid[0].len();
    if m == 0 {
        return 0
    }

    let mut visited = vec![vec![false; m]; n];
    let mut counter = 0;

    fn visit(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize, n: usize, m: usize) {
        if i < 0 || i >= n || j < 0 || j >= m || visited[i][j] || grid[i][j] == '0' {
            return
        }
        visited[i][j] = true;

        if i > 0 {
            visit(grid, visited, i-1, j, n, m);
        }
        if j > 0 {
            visit(grid, visited, i, j-1, n, m);
        }
        visit(grid, visited, i+1, j, n, m);
        visit(grid, visited, i, j+1, n, m);
    }

    for i in 0..n {
        for j in 0..m {
            if visited[i][j] || grid[i][j] == '0' {
                continue
            }
            counter += 1;
            visit(&grid, &mut visited, i, j, n, m);
        }
    }
    counter
}