struct NumArray {
    nums: Vec<i32>,
    cumsum: Vec<i32>,
}

impl NumArray {
    fn new(mut nums: Vec<i32>) -> Self {
        let mut cumsum = vec![0];
        for (i, &num) in nums.iter().enumerate() {
            cumsum.push(cumsum[i] + num);
        }
        Self {
            nums,
            cumsum,
        }
    }

    fn update(&mut self, i: i32, val: i32) {
        let i = i as usize;
        for j in i + 1..self.cumsum.len() {
            self.cumsum[j] += val - self.nums[i];
        }
        self.nums[i] = val;
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.cumsum[j as usize + 1] - self.cumsum[i as usize]
    }
}