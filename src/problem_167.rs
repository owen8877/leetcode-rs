pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let n = numbers.len();
    if n == 2 {
        return vec![1, 2]
    }
    let mut left = 0;
    let mut right = n-1;
    let mut right_decrease = numbers[left] + numbers[right] > target;
    loop {
        if right_decrease {
            while numbers[left] + numbers[right] > target && left < right {
                right -= 1;
            }
            right_decrease = false;
        } else {
            while numbers[left] + numbers[right] < target && left < right {
                left += 1;
            }
            right_decrease = true;
        }

        if numbers[left] + numbers[right] == target {
            return vec![left as i32 + 1, right as i32 + 1]
        }
    }
    vec![left as i32, right as i32]
}