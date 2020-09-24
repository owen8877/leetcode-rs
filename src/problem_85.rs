pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    use std::cmp::*;
    pub fn largest_rectangle_area(heights: &[i32]) -> i32 {
        let n = heights.len();
        let mut first_left_smaller = vec![-1; n];
        let mut first_right_smaller = vec![n as i32; n];
        let mut increasing_before = vec![];
        for (i, &num) in heights.into_iter().enumerate() {
            while let Some(&left) = increasing_before.last() {
                let left = left as usize;
                if heights[left] > num {
                    first_right_smaller[left] = i as i32;
                    increasing_before.pop();
                } else {
                    break
                }
            }
            for j in (0..increasing_before.len()).rev() {
                let index = increasing_before[j];
                if heights[index] < num {
                    first_left_smaller[i] = index as i32;
                    break
                }
            }
            increasing_before.push(i);
        }

        (0..n).fold(0, |max_area, i| {
            let left_index = first_left_smaller[i];
            let right_index = first_right_smaller[i];
            max(max_area, heights[i] * (right_index-left_index-1))
        })
    }

    let n = matrix.len();
    if n == 0 {
        return 0
    }
    let m = matrix[0].len();
    let mut heights = vec![0; m];
    let mut ma = 0;
    for i in 0..n {
        for j in 0..m {
            match matrix[i][j] {
                '0' => heights[j] = 0,
                _ => heights[j] += 1,
            }
        }
        ma = max(ma, largest_rectangle_area(heights.as_slice()));
    }
    ma
}