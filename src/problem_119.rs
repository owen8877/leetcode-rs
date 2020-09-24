pub fn get_row(r: i32) -> Vec<i32> {
    let r = r as usize;
    let mut result = Vec::with_capacity(r+1);
    let mut c = 1;
    result.push(c as i32);
    for i in 0..r {
        c *= r - i;
        c /= i + 1;
        result.push(c as i32);
    }
    result
}