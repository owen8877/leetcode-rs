pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let n = matrix.len();
    if n == 0 {
        return false;
    }
    let m = matrix[0].len();
    if m == 0 {
        return false;
    }
    let mut i = 0;
    let mut j = m - 1;
    while i < n && j >= 0 {
        if matrix[i][j] == target {
            return true;
        } else if matrix[i][j] > target {
            if j == 0 {
                return false;
            }
            j -= 1;
        } else {
            i += 1;
        }
    }
    false
}