pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    if n == 1 {
        return vec![vec![1]]
    }
    let n = n as usize;
    let ds = [[0, 1], [1, 0], [0, -1], [-1, 0]];
    let mut d_index = 0;
    let mut result = vec![vec![1; n]; n];
    let mut i = 0;
    let mut j = 0;
    let mut right = n-1;
    let mut left = 0;
    let mut top = n-1;
    let mut bottom = 0;
    let mut counter = 2;

    while right >= left && top >= bottom {
        let d = ds[d_index];
        i = (i as i32 + d[0]) as usize;
        j = (j as i32 + d[1]) as usize;
        result[i][j] = counter;
        counter += 1;
        match d_index {
            0 => if j == right {
                d_index = 1;
                bottom += 1;
            },
            1 => if i == top {
                d_index = 2;
                right -= 1;
            },
            2 => if j == left {
                d_index = 3;
                top -= 1;
            },
            3 => if i == bottom {
                d_index = 0;
                left += 1;
            },
            _ => unreachable!(),
        }
    }
    result
}