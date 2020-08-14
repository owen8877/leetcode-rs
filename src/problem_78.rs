pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn core(nums: &[i32]) -> Vec<Vec<i32>> {
        match nums.len() {
            0 => vec![vec![]],
            1 => vec![vec![], vec![nums[0]]],
            n => {
                let mut result = vec![];
                for mut v in core(&nums[0..n-1]) {
                    result.push(v.clone());
                    v.push(nums[n-1]);
                    result.push(v);
                }
                result
            },
        }
    }

    core(nums.as_slice())
}