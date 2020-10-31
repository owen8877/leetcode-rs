struct NumMatrix {
    always0: bool,
    cumsum: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let n = matrix.len();
        if n == 0 {
            return Self {
                always0: true,
                cumsum: vec![],
            };
        }
        let m = matrix[0].len();
        if m == 0 {
            return Self {
                always0: true,
                cumsum: vec![],
            };
        }

        let mut cumsum = vec![vec![0; m + 1]; n + 1];
        for i in 0..n {
            let mut h = 0;
            for j in 0..m {
                h += matrix[i][j];
                cumsum[i + 1][j + 1] = h + cumsum[i][j + 1];
            }
        }

        Self {
            always0: false,
            cumsum,
        }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        if self.always0 {
            return 0;
        }
        let row1 = row1 as usize;
        let row2 = row2 as usize;
        let col1 = col1 as usize;
        let col2 = col2 as usize;
        self.cumsum[row1][col1] + self.cumsum[row2 + 1][col2 + 1] - self.cumsum[row1][col2 + 1] - self.cumsum[row2 + 1][col1]
    }
}