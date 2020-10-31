struct NumArray {
    cumsum: Vec<i32>,
}

impl NumArray {
    fn new(mut nums: Vec<i32>) -> Self {
        let mut cumsum = vec![0];
        for (i, &num) in nums.iter().enumerate() {
            cumsum.push(cumsum[i] + num);
        }
        Self {
            cumsum,
        }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.cumsum[j as usize + 1] - self.cumsum[i as usize]
    }
}