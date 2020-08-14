pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let n = heights.len();
    let mut first_left_smaller: Vec<Option<usize>> = vec![None; n];
    let mut first_right_smaller: Vec<Option<usize>> = vec![None; n];
    let mut increasing_before = vec![];
    for (i, &num) in heights.iter().enumerate() {
        while let Some(left) = increasing_before.pop() {
            if heights[left] > num {
                first_right_smaller[left] = Some(i);
            } else if heights[left] <= num {
                increasing_before.push(left);
                break
            }
        }
        for j in (0..increasing_before.len()).rev() {
            let index = increasing_before[j];
            if heights[index] < num {
                first_left_smaller[i] = Some(index);
                break
            }
        }
        increasing_before.push(i);
    }

    (0..n).fold(0, |max_area, i| {
        let left_index = first_left_smaller[i].map_or(-1, |k| k as i32);
        let right_index = first_right_smaller[i].unwrap_or(n) as i32;
        std::cmp::max(max_area, heights[i] * (right_index-left_index-1))
    })
}

#[test]
fn test_largest_rectangle_area() {
    assert_eq!(largest_rectangle_area(vec![5,5,1,7,1,1,5,2,7,6]), 12);
    assert_eq!(largest_rectangle_area(vec![1,2,2]), 4);
}