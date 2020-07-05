pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let m = matrix.len();
    if m == 0 {
        return vec![]
    }
    let n = matrix[0].len();
    if n == 0 {
        return vec![]
    }

    if m == 1 {
        return matrix[0].clone()
    }
    if n == 1 {
        return matrix.iter().map(|v| v[0]).collect()
    }

    let ds = [[0, 1], [1, 0], [0, -1], [-1, 0]];
    let mut d_index = 0;
    let mut result = vec![matrix[0][0]];
    let mut i = 0;
    let mut j = 0;
    let mut right = n-1;
    let mut left = 0;
    let mut top = m-1;
    let mut bottom = 0;

    while right >= left && top >= bottom {
        let d = ds[d_index];
        i = (i as i32 + d[0]) as usize;
        j = (j as i32 + d[1]) as usize;
        result.push(matrix[i][j]);
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

#[test]
fn test_spiral_order() {
    assert_eq!(spiral_order(vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9,10,11,12],
    ]), vec![1,2,3,4,8,12,11,10,9,5,6,7]);
    assert_eq!(spiral_order(vec![vec![1]]), vec![1]);
    assert_eq!(spiral_order(vec![vec![1], vec![2], vec![3]]), vec![1, 2, 3]);
    assert_eq!(spiral_order(vec![vec![1, 2, 3, 4]]), vec![1,2,3,4]);
}