pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
    if m == 0 {
        0
    } else {
        let mut mask = 1 << 30;
        let mut sum = 0;
        for i in 0..30 {
            if m & mask != n & mask {
                return sum << (30-i);
            }
            sum += (m & mask) >> (30 - i);
            sum <<= 1;
            mask >>= 1;
        }
        m
    }
}