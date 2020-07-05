pub fn trap(height: Vec<i32>) -> i32 {
    fn core(height: &[i32], direction: bool) -> i32 {
        if height.len() <= 2 {
            return 0
        }
        let mut sum = 0;
        if direction {
            for i in 1..height.len() {
                if height[i] >= height[0] {
                    return sum + core(&height[i..], true)
                }
                sum += height[0] - height[i];
            }
            core(height, false)
        } else {
            for i in (0..height.len()-1).rev() {
                if height[i] >= height[height.len()-1] {
                    return sum + core(&height[..=i], false)
                }
                sum += height[height.len()-1] - height[i];
            }
            core(height, true)
        }
    }
    core(&height[..], true)
}

#[test]
fn test_trap() {
    assert_eq!(trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
}