pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let n = matrix.len();
    if n == 0 {
        return 0;
    }
    let m = matrix[0].len();
    if m == 0 {
        return 0;
    }
    let mut count = vec![vec![0; m + 1]; n + 1];
    for i in 0..n {
        let mut h = 0;
        for j in 0..m {
            h += (matrix[i][j] as u8 - '0' as u8) as usize;
            count[i + 1][j + 1] = count[i][j + 1] + h;
        }
    }
    let mut largest = 0;
    for i in 0..n {
        for j in 0..m {
            for k in 0..=std::cmp::min(i, j) {
                if count[i + 1][j + 1] + count[i - k][j - k] - count[i + 1][j - k] - count[i - k][j + 1] == (k + 1).pow(2) {
                    largest = std::cmp::max(largest, (k + 1).pow(2));
                } else {
                    break;
                }
            }
        }
    }
    largest as i32
}