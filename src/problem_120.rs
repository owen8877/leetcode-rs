pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let n = triangle.len();
    let mut min_sum = triangle[n-1].clone();
    for i in (0..n-1).rev() {
        let mut m = vec![0; i+1];
        for j in 0..=i {
            m[j] = std::cmp::min(min_sum[j], min_sum[j+1]) + triangle[i][j];
        }
        min_sum = m;
    }
    min_sum[0]
}