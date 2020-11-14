pub fn count_bits(num: i32) -> Vec<i32> {
    let mut result = vec![0; num as usize + 1];
    for i in 0..=num as usize {
        let mut j = i;
        while j > 0 {
            result[i] += (j & 1) as i32;
            j /= 2;
        }
    }
    result
}