pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    (0..num_rows as usize).map(|r| {
        let mut result = Vec::with_capacity(r+1);
        let mut c = 1;
        result.push(c as i32);
        for i in 0..r {
            c *= r - i;
            c /= i + 1;
            result.push(c as i32);
        }
        result
    }).collect()
}