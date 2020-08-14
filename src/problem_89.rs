pub fn gray_code(n: i32) -> Vec<i32> {
    if n == 0 {
        return vec![0]
    }
    let m = 2usize.pow(n as u32);
    let mut graycode = vec![0; m];
    graycode[1] = 1;
    let mut delta = 2;
    for i in 1..n as usize {
        for j in 0..delta {
            graycode[delta*2-1-j] = graycode[j] + delta as i32;
        }
        delta *= 2;
    }
    graycode
}
