pub fn num_squares(n: i32) -> i32 {
    fn core(n: usize, m: usize, nums: usize, mut min_nums: usize) -> usize {
        if nums >= min_nums {
            return min_nums;
        }
        if n == 0 {
            return std::cmp::min(nums, min_nums);
        }
        if m == 0 {
            return min_nums;
        }

        let allowance = n / (m * m);
        for i in (0..=allowance).rev() {
            if nums + i < min_nums {
                min_nums = core(n - i * m * m, m - 1, nums + i, min_nums);
            }
        }
        min_nums
    }

    core(n as usize, (n as f64).sqrt().ceil() as usize, 0, n as usize) as i32
}