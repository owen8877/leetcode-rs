pub fn hamming_weight(n: u32) -> i32 {
    let mut mask = 1;
    let mut sum = 0;
    for i in 0..32 {
        sum += (n & mask) >> i;
        mask <<= 1;
    }
    sum as i32
}