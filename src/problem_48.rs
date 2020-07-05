pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    if n <= 1 {
        return
    }
    if n % 2 == 0 {
        let m = n/2;
        for i in 0..m {
            for j in 0..m {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[n-1-j][i];
                matrix[n-1-j][i] = matrix[n-1-i][n-1-j];
                matrix[n-1-i][n-1-j] = matrix[j][n-1-i];
                matrix[j][n-1-i] = tmp;
            }
        }
    } else {
        let m = n/2;
        for i in 0..m {
            for j in 0..m+1 {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[n-1-j][i];
                matrix[n-1-j][i] = matrix[n-1-i][n-1-j];
                matrix[n-1-i][n-1-j] = matrix[j][n-1-i];
                matrix[j][n-1-i] = tmp;
            }
        }
    }
}