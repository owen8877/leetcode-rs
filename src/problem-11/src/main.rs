pub fn max_area_ori(height: Vec<i32>) -> i32 {
    use std::cmp::{max, min};

    let mut largest = min(height[1], height[0]);
    for i in 0..height.len()-1 {
        if height[i] == 0 {
            continue
        }
        for j in max(i+1, (largest / height[i]) as usize)..height.len() {
            let volume = (j-i) as i32 * min(height[i], height[j]);
            largest = max(largest, volume);
        }
    }

    largest
}

pub fn max_area_std(height: Vec<i32>) -> i32 {
    use std::cmp::{max, min};

    let n = height.len();
    let mut largest = min(height[n-1], height[0]) * (n-1) as i32;
    let mut left = 0;
    let mut right = n-1;
    while left < right {
        largest = max(largest, min(height[right], height[left]) * (right-left) as i32);
        if height[left] > height[right] {
            right -= 1;
        } else {
            left += 1;
        }
    }

    largest
}

#[test]
fn test_max_area() {
    assert_eq!(max_area_std(vec![1,8,6,2,5,4,8,3,7]), 49);
    assert_eq!(max_area_std(vec![1,8,6,2,0,4,8,3,7]), 49);
}
