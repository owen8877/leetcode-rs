pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    if n == 0 {
        return
    }
    let m = matrix[0].len();
    if m == 0 {
        return
    }
    let mut flag_col = vec![false; n];
    let mut flag_row = vec![false; m];
    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == 0 {
                flag_col[i] = true;
                break
            }
        }
    }
    for j in 0..m {
        for i in 0..n {
            if matrix[i][j] == 0 {
                flag_row[j] = true;
                break
            }
        }
    }
    for i in 0..n {
        if flag_col[i] {
            for j in 0..m {
                matrix[i][j] = 0;
            }
        }
    }
    for j in 0..m {
        if flag_row[j] {
            for i in 0..n {
                matrix[i][j] = 0;
            }
        }
    }
}