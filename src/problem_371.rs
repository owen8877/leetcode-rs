pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
    let mut ret = 0;
    let mut acc = 1;
    while acc != 0 {
        ret = a ^ b;
        acc = ((a as u32 & b as u32) << 1) as i32;
        a = ret;
        b = acc;
    }
    ret
}